//! ШАГИ 2–3 (и часть шага 4). Обслуживание одного соединения.
//!
//! Схема, к которой мы придём:
//!   * сокет делим на две половинки (`into_split`);
//!   * задача-писатель тянет из `mpsc::Receiver` и пишет в сокет;
//!   * текущая задача читает из сокета и меняет состояние комнаты.
//!
//! Так мы избегаем `Mutex<TcpStream>`: writer — единственный владелец
//! пишущей половинки, а «все, кто хочет что-то сказать этому клиенту»,
//! просто кладут сообщение в его очередь.
//!
//! ГОТОВО в заготовке: split, рукопожатие с таймаутом, валидация ника.
//! ПИШЕМ на занятии: регистрация в комнате, задача-писатель, цикл чтения.

use std::net::SocketAddr;
use std::time::Duration;

#[allow(unused_imports)]
use netchat_protocol::{normalize_nick, read_msg, write_msg, ClientMsg, ProtoError, ServerMsg};
use tokio::io::{BufReader, BufWriter};
use tokio::net::TcpStream;
#[allow(unused_imports)]
use tokio::sync::mpsc; use tokio_util::sync::CancellationToken;
#[allow(unused_imports)]
use tracing::{debug, info, warn};

use crate::state::SharedRoom;

/// Сколько ждём `Hello` от нового соединения.
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn handle(
    stream: TcpStream,
    addr: SocketAddr,
    room: SharedRoom,
    shutdown: CancellationToken,
) -> anyhow::Result<()> {
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    // --- 1. Рукопожатие (готово) -------------------------------------------
    // Таймаут обязателен: иначе молчащее соединение будет вечно держать
    // задачу и файловый дескриптор.
    let hello = tokio::time::timeout(HANDSHAKE_TIMEOUT, read_msg::<_, ClientMsg>(&mut reader))
        .await
        .map_err(|_| anyhow::anyhow!("клиент {addr} не представился за {HANDSHAKE_TIMEOUT:?}"))??;

    let nick = match hello {
        ClientMsg::Hello { nick } => match normalize_nick(&nick) {
            Some(nick) => nick,
            None => {
                let err = ServerMsg::Error { text: "недопустимый ник".into() };
                write_msg(&mut writer, &err).await?;
                return Ok(());
            }
        },
        other => {
            let err = ServerMsg::Error { text: format!("ожидался Hello, получено {other:?}") };
            write_msg(&mut writer, &err).await?;
            return Ok(());
        }
    };

    // --- 2. Регистрация в комнате ------------------------------------------
    // TODO(шаг 2):
    //   1) создать канал: let (tx, mut rx) = mpsc::unbounded_channel::<ServerMsg>();
    //   2) под локом позвать room.join(&nick, tx.clone());
    //      лок брать в отдельном блоке `{ ... }`, чтобы он не жил дольше нужного;
    //   3) если ник занят — написать клиенту ServerMsg::Error и выйти.

    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMsg>();

    let joined = {
        let mut guard = room.lock().expect("поток заблокирован");
        guard.join(&nick, tx.clone())
    };
    
    if !joined {
        let err = ServerMsg::Error { text: "ник занят".into() };
        write_msg(&mut writer, &err).await?;
        return Ok(());
    }
    info!(%addr, %nick, "клиен подключен к чату");

    // --- 3. Задача-писатель -------------------------------------------------
    // TODO(шаг 2): tokio::spawn(async move { while let Some(msg) = rx.recv().await { ... } })
    //   Внутри — write_msg(&mut writer, &msg); при ошибке записи выходим из цикла.
    //   `writer` переезжает в эту задачу — больше нигде его использовать нельзя.

    let writer_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = write_msg(&mut writer, &msg).await {
                warn!(%addr, error = %e, "не получилось записать в сокет");
                break;
            }
        }
    });

    // --- 3b. Приветствие и объявление остальным ----------------------------
    // TODO(шаг 2): под локом отправить себе Welcome, остальным — System("... присоединился").

    // Фигурные скобки обязательны: MutexGuard не Send, и если он доживёт
    // до `.await` ниже, вся future перестанет быть Send.
    {
        let lock = room.lock().expect("поток заблокирован");
        let motd = format!("ник {}, на сервере {}, где {} пользователей", nick, addr, lock.len());
        lock.send_to(&nick, ServerMsg::Welcome { nick: nick.clone(), motd });
        lock.broadcast(&ServerMsg::System { text: format!("{nick} присоединился к чату") }, Some(&nick));
    }

    // --- 4. Основной цикл чтения -------------------------------------------
    let loop_result = read_loop(&mut reader, &room, &nick, &shutdown).await;

    // --- 5. Уборка ----------------------------------------------------------
    // TODO(шаг 2): выполняется на ЛЮБОМ пути выхода — обрыв связи, ошибка,
    //   /quit, shutdown:
    //   1) room.leave(&nick) + broadcast System("... вышел");
    //   2) drop(tx) — уронив последний Sender, мы завершаем задачу-писателя;
    //   3) дождаться writer_task.await, чтобы буфер успел уйти в сокет.
    {
        let mut guard = room.lock().expect("поток заблокирован");
        guard.leave(&nick);
        guard.broadcast(&ServerMsg::System { text: format!("{nick} вышел из чата") }, Some(&nick));
    }
    drop(tx);
    let _ = writer_task.await;
    info!(%addr, %nick, "участник вышел");

    loop_result
}

/// Читаем сообщения, пока клиент не уйдёт или сервер не начнёт выключаться.
async fn read_loop(
    reader: &mut BufReader<tokio::net::tcp::OwnedReadHalf>,
    room: &SharedRoom,
    nick: &str,
    shutdown: &CancellationToken,
) -> anyhow::Result<()> {
    loop {
        // TODO(шаг 4): обернуть чтение в `tokio::select!` с веткой
        //   `_ = shutdown.cancelled() => { ... }`, чтобы соединение
        //   отпускалось при остановке сервера. Пока просто читаем.
        //
        // Про cancel safety: `read_msg` НЕ cancel-safe — если её отменить
        // посередине кадра, прочитанные байты потеряются. Обсуждаем на шаге 4,
        // почему в нашем случае это допустимо.
        let _ = shutdown;
        let incoming = read_msg::<_, ClientMsg>(reader).await;

        match incoming {
            Ok(ClientMsg::Bye) => return Ok(()),

            Ok(ClientMsg::Text { body }) => {
                // TODO(шаг 2): пустые сообщения игнорируем,
                //   остальное — broadcast всем, кроме автора.
                let body = body.trim().to_owned();
                if body.is_empty() {
                    continue;
                }
                let guard_room = room.lock().expect("поток заблокирован");
                guard_room.broadcast(
                    &ServerMsg::Chat { from: nick.to_owned(), body }, 
                    Some(nick),
                );
            }

            Ok(ClientMsg::Private { to, body }) => {
                // TODO(шаг 3): доставить одному; если ника нет — вернуть
                //   автору ServerMsg::Error.
                let guard_room = room.lock().expect("поток заблокирован");
                let approved = guard_room.send_to(
                    &to,
                    ServerMsg::Chat { from: nick.to_owned(), body }
                );
                if !approved {
                    guard_room.send_to(
                        nick, 
                        ServerMsg::Error { text: format!("Пользователь {to} сейчас не в чате") }
                    );
                }
            }

            Ok(ClientMsg::ListUsers) => {
                // TODO(шаг 3): отправить автору ServerMsg::Users.
                let guard_room = room.lock().expect("поток заблокирован");
                let users = guard_room.users();
                guard_room.send_to(nick, ServerMsg::Users { nicks: users });
            }

            Ok(ClientMsg::Hello { .. }) => {
                // TODO(шаг 3): второй Hello — это ошибка клиента, но не повод
                //   рвать соединение. Отвечаем ServerMsg::Error.
                let guard_room = room.lock().expect("поток заблокирован");
                guard_room.send_to(nick, ServerMsg::Error { text: "повторный Hello".into() });
            }

            // Штатный уход клиента — это не ошибка сервера.
            Err(ProtoError::Closed) => return Ok(()),

            Err(e) => {
                warn!(%nick, error = %e, "протокольная ошибка, закрываем соединение");
                return Ok(());
            }
        }
    }
}
