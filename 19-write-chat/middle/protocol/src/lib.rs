//! Общий протокол клиента и сервера NetChat.
//!
//! Кадрирование (framing): `[u32 BE — длина полезной нагрузки][bincode-байты]`.
//! TCP — это поток байт, а не поток сообщений, поэтому границы сообщений
//! приходится задавать самим. Здесь — префиксом длины.
//!

//// | service_1 | --------> | service_2 |
//// | Message   |          | Message   |
////                REST API
////             |   JSON   |

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Верхняя граница размера кадра. Без неё злой клиент пришлёт len = 4 GiB
/// и сервер попытается выделить под это буфер.
pub const MAX_FRAME_LEN: u32 = 1 << 20; // 1 MiB

/// Сообщения, которые клиент отправляет серверу.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientMsg {
    /// Первое сообщение в соединении: представиться.
    Hello { nick: String },
    /// Сообщение в общий чат.
    Text { body: String },
    /// Личное сообщение конкретному пользователю.
    Private { to: String, body: String },
    /// Запросить список пользователей.
    ListUsers,
    /// Корректный выход.
    Bye,
}

/// Сообщения, которые сервер отправляет клиенту.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerMsg {
    Welcome { nick: String, motd: String },
    Chat { from: String, body: String },
    Private { from: String, body: String },
    System { text: String },
    Users { nicks: Vec<String> },
    Error { text: String },
}

#[derive(Debug, thiserror::Error)]
pub enum ProtoError {
    #[error("соединение закрыто собеседником")]
    Closed,
    #[error("кадр слишком большой: {0} байт (максимум {MAX_FRAME_LEN})")]
    FrameTooLarge(u32),
    #[error("ошибка ввода-вывода: {0}")]
    Io(#[from] std::io::Error),
    #[error("не удалось (де)сериализовать кадр: {0}")]
    Codec(#[from] bincode::Error),
}

/// Записать одно сообщение в поток.
///
/// `flush` обязателен: под `BufWriter`/`OwnedWriteHalf` данные иначе могут
/// остаться в буфере, и собеседник будет ждать вечно.
pub async fn write_msg<W, T>(writer: &mut W, msg: &T) -> Result<(), ProtoError>
where
    W: AsyncWrite + Unpin,
    T: Serialize,
{
    let payload = bincode::serialize(msg)?;
    let len = u32::try_from(payload.len()).map_err(|_| ProtoError::FrameTooLarge(u32::MAX))?;
    if len > MAX_FRAME_LEN {
        return Err(ProtoError::FrameTooLarge(len));
    }
    writer.write_u32(len).await?;
    writer.write_all(&payload).await?;
    writer.flush().await?;
    Ok(())
}

/// Прочитать ровно одно сообщение из потока.
///
/// Возвращает [`ProtoError::Closed`], если собеседник закрыл соединение
/// на границе кадров — это штатное завершение, а не ошибка.
pub async fn read_msg<R, T>(r: &mut R) -> Result<T, ProtoError>
where
    R: AsyncRead + Unpin,
    T: DeserializeOwned,
{
    let len = match r.read_u32().await {
        Ok(len) => len,
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Err(ProtoError::Closed),
        Err(e) => return Err(e.into()),
    };
    if len > MAX_FRAME_LEN {
        return Err(ProtoError::FrameTooLarge(len));
    }
    let mut buf = vec![0u8; len as usize];
    // read_exact, а не read: одно логическое сообщение может приехать
    // несколькими TCP-сегментами.
    r.read_exact(&mut buf).await?;
    Ok(bincode::deserialize(&buf)?)
}

/// Нормализация ника: обрезаем пробелы, ограничиваем длину, запрещаем пустой.
pub fn normalize_nick(raw: &str) -> Option<String> {
    let nick: String = raw.trim().chars().take(24).collect();
    if nick.is_empty() || nick.contains(char::is_whitespace) {
        None
    } else {
        Some(nick)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn roundtrip_single_message() {
        let mut buf: Vec<u8> = Vec::new();
        let msg = ClientMsg::Hello { nick: "alice".into() };
        write_msg(&mut buf, &msg).await.unwrap();

        let mut cursor = std::io::Cursor::new(buf);
        let got: ClientMsg = read_msg(&mut cursor).await.unwrap();
        assert_eq!(got, msg);
    }

    #[tokio::test]
    async fn two_messages_in_one_buffer_keep_their_boundaries() {
        let mut buf: Vec<u8> = Vec::new();
        write_msg(&mut buf, &ClientMsg::Text { body: "раз".into() }).await.unwrap();
        write_msg(&mut buf, &ClientMsg::Text { body: "два".into() }).await.unwrap();

        let mut cursor = std::io::Cursor::new(buf);
        let a: ClientMsg = read_msg(&mut cursor).await.unwrap();
        let b: ClientMsg = read_msg(&mut cursor).await.unwrap();
        assert_eq!(a, ClientMsg::Text { body: "раз".into() });
        assert_eq!(b, ClientMsg::Text { body: "два".into() });
    }

    #[tokio::test]
    async fn empty_stream_is_reported_as_closed() {
        let mut cursor = std::io::Cursor::new(Vec::<u8>::new());
        let err = read_msg::<_, ClientMsg>(&mut cursor).await.unwrap_err();
        assert!(matches!(err, ProtoError::Closed));
    }

    #[tokio::test]
    async fn oversized_length_prefix_is_rejected_without_allocating() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&u32::MAX.to_be_bytes());
        let mut cursor = std::io::Cursor::new(buf);
        let err = read_msg::<_, ClientMsg>(&mut cursor).await.unwrap_err();
        assert!(matches!(err, ProtoError::FrameTooLarge(_)));
    }

    #[test]
    fn nick_validation() {
        assert_eq!(normalize_nick("  bob "), Some("bob".to_string()));
        assert_eq!(normalize_nick(""), None);
        assert_eq!(normalize_nick("two words"), None);
    }
}
