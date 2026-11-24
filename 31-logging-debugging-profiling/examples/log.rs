use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::init();

    trace!("Подробная трассировка");
    debug!("Данные для отладки");
    info!("Информационное сообщение");
    warn!("Предупреждение");
    error!("Сообщение об ошибке");
}
