use clap::Parser;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "netchat-server", about = "Учебный TCP-чат: сервер")]
struct Args {
    /// Адрес и порт для прослушивания
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    addr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with_target(false)
        .init();

    let args = Args::parse();
    let listener = TcpListener::bind(&args.addr).await?;

    let shutdown = CancellationToken::new();
    // Отдельная задача: ловим Ctrl+C и переводим его в отмену токена.
    {
        let shutdown = shutdown.clone();
        tokio::spawn(async move {
            if tokio::signal::ctrl_c().await.is_ok() {
                tracing::info!("Ctrl+C");
                shutdown.cancel();
            }
        });
    }

    netchat_server::run(listener, shutdown).await
}
