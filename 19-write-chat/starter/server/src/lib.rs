//! Ядро сервера вынесено в библиотеку, чтобы его можно было поднять
//! из интеграционного теста, а не только из `main`. Это не косметика:
//! без этого шаг 5 (тесты сети) не сделать.

pub mod conn;
pub mod state;

use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
#[allow(unused_imports)]
use tokio_util::task::TaskTracker;
use tracing::{info, warn};

/// Принимать соединения, пока не сработает `shutdown`.
///
/// ЦЕЛЬ шага 4: возврат из этой функции должен означать, что **все** задачи
/// соединений уже завершились. Сейчас это не так.
pub async fn run(listener: TcpListener, shutdown: CancellationToken) -> anyhow::Result<()> {
    let room = state::shared_room();

    // TODO(шаг 4): let tracker = TaskTracker::new();

    info!(addr = %listener.local_addr()?, "сервер слушает");

    loop {
        // TODO(шаг 4): завернуть accept в `tokio::select!` со второй веткой
        //   `_ = shutdown.cancelled() => break`.
        let accepted = listener.accept().await;

        let (stream, addr) = match accepted {
            Ok(pair) => pair,
            // Например, EMFILE: не валим весь сервер из-за одного accept.
            Err(e) => {
                warn!(error = %e, "accept не удался");
                continue;
            }
        };
        let _ = stream.set_nodelay(true);

        let room = room.clone();
        let shutdown = shutdown.clone();
        // TODO(шаг 4): заменить tokio::spawn на tracker.spawn.
        tokio::spawn(async move {
            if let Err(e) = conn::handle(stream, addr, room, shutdown).await {
                warn!(%addr, error = %e, "соединение завершилось с ошибкой");
            }
        });
    }

    // TODO(шаг 4): после выхода из цикла —
    //   tracker.close();
    //   tracker.wait().await;
    //   info!("все соединения закрыты");
    //   Ok(())
}
