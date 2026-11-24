//! Ядро сервера вынесено в библиотеку, чтобы его можно было поднять
//! из интеграционного теста, а не только из `main`. Это не косметика:
//! без этого шаг 5 (тесты сети) не сделать.

pub mod conn;
pub mod state;

use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tracing::{info, warn};

/// Принимать соединения, пока не сработает `shutdown`.
///
/// Возврат из этой функции означает, что **все** задачи соединений
/// уже завершились.
pub async fn run(listener: TcpListener, shutdown: CancellationToken) -> anyhow::Result<()> {
    let room = state::shared_room();
    let tracker = TaskTracker::new();

    info!(addr = %listener.local_addr()?, "сервер слушает");

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => break,
            accepted = listener.accept() => {

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
                tracker.spawn(async move {
                    if let Err(e) = conn::handle(stream, addr, room, shutdown).await {
                        warn!(%addr, error = %e, "соединение завершилось с ошибкой");
                    }
                });
            }
        }
    }

    tracker.close();
    tracker.wait().await;
    info!("все соединения закрыты");
    Ok(())
}
