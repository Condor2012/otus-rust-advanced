use tracing::{Level, error, info, instrument, warn};
use tracing_subscriber::FmtSubscriber;

#[instrument]
fn process(user_id: u64, items: usize) {
    info!(items, "Обработка запроса");
    if items == 0 {
        warn!("Пустой список элементов");
    }
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("подписчик устанавливается один раз");

    process(42, 3);
    process(7, 0);
    error!(code = 500, "Пример ошибки со структурированным полем");
}
