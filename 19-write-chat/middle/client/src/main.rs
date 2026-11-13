//! ШАГ 2 (вторая половина). Консольный клиент.
//!
//! Главная идея: `tokio::select!` в одном цикле ждёт сразу три события —
//! строку из stdin, сообщение из сокета и Ctrl+C. Это ровно тот случай,
//! для которого `select!` придуман: несколько независимых источников событий,
//! и мы не знаем, какой выстрелит первым.
//!
//! ГОТОВО в заготовке: разбор аргументов, парсер команд, отрисовка сообщений.
//! ПИШЕМ на занятии: собственно цикл `select!`.

use anyhow::Context;
use clap::Parser;
#[allow(unused_imports)]
use netchat_protocol::{read_msg, write_msg, ClientMsg, ProtoError, ServerMsg};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;

#[derive(Parser, Debug)]
#[command(name = "netchat-client", about = "Учебный TCP-чат: клиент")]
struct Args {
    /// Адрес сервера
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    addr: String,
    /// Ник в чате
    #[arg(short, long)]
    nick: String,
}

const HELP: &str = "\
Команды:
  <текст>              отправить в общий чат
  /w <ник> <текст>     личное сообщение
  /list                кто в чате
  /help                эта справка
  /quit                выйти";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let stream = TcpStream::connect(&args.addr)
        .await
        .with_context(|| format!("не удалось подключиться к {}", args.addr))?;
    let (net_read, mut net_write) = stream.into_split();
    let mut net_read = BufReader::new(net_read);

    write_msg(&mut net_write, &ClientMsg::Hello { nick: args.nick.clone() }).await?;

    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    println!("{HELP}\n");

    // TODO(шаг 2): главный цикл.
    //
    // loop {
    //     tokio::select! {
    //         // 1. Пользователь что-то набрал.
    //         line = stdin.next_line() => { ... parse_input(&line) ... }
    //         // 2. Сервер что-то прислал.
    //         incoming = read_msg::<_, ServerMsg>(&mut net_read) => { ... render(&msg) ... }
    //         // 3. Ctrl+C — прощаемся вежливо.
    //         _ = tokio::signal::ctrl_c() => { ... ClientMsg::Bye ... }
    //     }
    // }
    //
    // Подсказки:
    //   * `stdin.next_line()` -> io::Result<Option<String>>; None = Ctrl+D;
    //   * ProtoError::Closed из read_msg — сервер закрылся, это не паника;
    //   * ветки select! пишем так, чтобы `net_write` не одалживался дважды.
    let _ = (&mut stdin, &mut net_read, &mut net_write);
    todo!("главный цикл клиента");

    #[allow(unreachable_code)]
    {
        println!("-- до связи");
        Ok(())
    }
}

#[allow(dead_code)]
enum Input {
    Nothing,
    Help,
    Quit,
    Msg(ClientMsg),
    Bad(&'static str),
}

/// Готово: превращает строку из stdin в намерение пользователя.
#[allow(dead_code)]
fn parse_input(line: &str) -> Input {
    let line = line.trim();
    if line.is_empty() {
        return Input::Nothing;
    }
    if !line.starts_with('/') {
        return Input::Msg(ClientMsg::Text { body: line.to_owned() });
    }
    let mut parts = line.splitn(3, ' ');
    match parts.next().unwrap_or_default() {
        "/quit" | "/exit" => Input::Quit,
        "/help" => Input::Help,
        "/list" => Input::Msg(ClientMsg::ListUsers),
        "/w" | "/msg" => match (parts.next(), parts.next()) {
            (Some(to), Some(body)) if !to.is_empty() && !body.trim().is_empty() => {
                Input::Msg(ClientMsg::Private { to: to.to_owned(), body: body.trim().to_owned() })
            }
            _ => Input::Bad("использование: /w <ник> <текст>"),
        },
        _ => Input::Bad("неизвестная команда, /help"),
    }
}

/// Готово: как показываем сообщения сервера.
#[allow(dead_code)]
fn render(msg: &ServerMsg) {
    match msg {
        ServerMsg::Welcome { nick, motd } => println!("== вы вошли как {nick} ({motd})"),
        ServerMsg::Chat { from, body } => println!("[{from}] {body}"),
        ServerMsg::Private { from, body } => println!("(лично от {from}) {body}"),
        ServerMsg::System { text } => println!("-- {text}"),
        ServerMsg::Users { nicks } => println!("== в чате: {}", nicks.join(", ")),
        ServerMsg::Error { text } => println!("!! {text}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_becomes_text_message() {
        assert!(matches!(parse_input("привет"), Input::Msg(ClientMsg::Text { .. })));
    }

    #[test]
    fn private_command_is_parsed() {
        match parse_input("/w bob привет тебе") {
            Input::Msg(ClientMsg::Private { to, body }) => {
                assert_eq!(to, "bob");
                assert_eq!(body, "привет тебе");
            }
            _ => panic!("не распознали /w"),
        }
    }

    #[test]
    fn private_without_body_is_rejected() {
        assert!(matches!(parse_input("/w bob"), Input::Bad(_)));
    }
}
