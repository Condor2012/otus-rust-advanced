use clap::Parser;
use netchat_server::run;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

#[derive(Parser, Debug)]
#[command(name = "netchat-server", about = "Учебный TCP-чат: сервер")]
struct Args {
    /// Адрес для прослушивания
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    listen: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "netchat_server=info".into()),
        )
        .init();

    let args = Args::parse();
    let listener = TcpListener::bind(&args.listen).await?;
    let shutdown = CancellationToken::new();

    // Отдельная задача ждёт Ctrl+C и будит основной цикл через токен.
    tokio::spawn({
        let shutdown = shutdown.clone();
        async move {
            let _ = tokio::signal::ctrl_c().await;
            shutdown.cancel();
        }
    });

    run(listener, shutdown).await
}
