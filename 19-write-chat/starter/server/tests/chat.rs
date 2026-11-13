//! ШАГ 5. Интеграционные тесты: поднимаем настоящий сервер на случайном порту
//! (`127.0.0.1:0` — ядро само выдаст свободный) и ходим в него настоящими
//! TCP-клиентами. Никаких моков и никаких `sleep` для синхронизации.
//!
//! ГОТОВО в заготовке: обёртки `Server` и `Client` + один тест-образец.
//! ПИШЕМ на занятии: тесты на приват и на graceful shutdown.

use std::time::Duration;

use netchat_protocol::{read_msg, write_msg, ClientMsg, ServerMsg};
use tokio::io::BufReader;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

/// Общий таймаут ожидания. Тест должен падать, а не висеть.
const T: Duration = Duration::from_secs(3);

struct Server {
    addr: std::net::SocketAddr,
    shutdown: CancellationToken,
    handle: JoinHandle<anyhow::Result<()>>,
}

impl Server {
    async fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let shutdown = CancellationToken::new();
        let handle = tokio::spawn(netchat_server::run(listener, shutdown.clone()));
        Server { addr, shutdown, handle }
    }

    /// Требует, чтобы `run()` вернулся сам, и вернулся быстро.
    async fn stop(self) {
        self.shutdown.cancel();
        tokio::time::timeout(T, self.handle)
            .await
            .expect("сервер не остановился за отведённое время")
            .unwrap()
            .unwrap();
    }
}

struct Client {
    rd: BufReader<OwnedReadHalf>,
    wr: OwnedWriteHalf,
}

#[allow(dead_code)]
impl Client {
    /// Подключиться и дождаться `Welcome`.
    async fn join(server: &Server, nick: &str) -> Self {
        let mut c = Client::raw(server).await;
        c.send(ClientMsg::Hello { nick: nick.into() }).await;
        match c.recv().await {
            ServerMsg::Welcome { nick: got, .. } => assert_eq!(got, nick),
            other => panic!("ожидали Welcome, получили {other:?}"),
        }
        c
    }

    /// Подключиться без рукопожатия.
    async fn raw(server: &Server) -> Self {
        let stream = TcpStream::connect(server.addr).await.unwrap();
        let (rd, wr) = stream.into_split();
        Client { rd: BufReader::new(rd), wr }
    }

    async fn send(&mut self, msg: ClientMsg) {
        write_msg(&mut self.wr, &msg).await.unwrap();
    }

    async fn recv(&mut self) -> ServerMsg {
        tokio::time::timeout(T, read_msg::<_, ServerMsg>(&mut self.rd))
            .await
            .expect("сообщение от сервера не пришло")
            .unwrap()
    }

    /// Пропускаем служебные `System` — нас интересует содержательное сообщение.
    async fn recv_meaningful(&mut self) -> ServerMsg {
        loop {
            match self.recv().await {
                ServerMsg::System { .. } => continue,
                other => return other,
            }
        }
    }

    /// «Пришло ли что-нибудь прямо сейчас?» — для проверок «ничего не пришло».
    async fn try_recv_now(&mut self) -> Option<ServerMsg> {
        tokio::time::timeout(Duration::from_millis(150), read_msg::<_, ServerMsg>(&mut self.rd))
            .await
            .ok()
            .map(|r| r.unwrap())
    }
}

/// Образец: читаем его вместе и дальше пишем остальные по аналогии.
#[tokio::test]
async fn message_from_one_client_reaches_the_other() {
    let server = Server::start().await;
    let mut alice = Client::join(&server, "alice").await;
    let mut bob = Client::join(&server, "bob").await;

    alice.send(ClientMsg::Text { body: "привет, чат".into() }).await;

    let got = bob.recv_meaningful().await;
    assert_eq!(
        got,
        ServerMsg::Chat { from: "alice".into(), body: "привет, чат".into() }
    );

    server.stop().await;
}

// TODO(шаг 5): отправитель не получает свою же реплику.
//   Подсказка: while let Some(msg) = alice.try_recv_now().await { ... }

// TODO(шаг 5): приват доходит только адресату (три клиента: alice, bob, eve).

// TODO(шаг 5): приват несуществующему нику возвращает ServerMsg::Error.

// TODO(шаг 5): /list возвращает отсортированный список ников.

/// TODO(шаг 5): главный тест на graceful shutdown.
/// Если `run()` не дожидается задач соединений — тест должен упасть по таймауту
/// внутри `stop()`, а не подвиснуть навсегда.
#[tokio::test]
#[ignore = "снять #[ignore] на шаге 4/5"]
async fn shutdown_completes_even_with_live_connections() {
    let server = Server::start().await;
    let _alice = Client::join(&server, "alice").await;
    let _bob = Client::join(&server, "bob").await;
    server.stop().await;
}
