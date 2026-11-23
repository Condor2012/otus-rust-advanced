//! Игровой движок с броском двадцатигранного кубика.

mod a;

pub use crate::math::random_number;

/// Бросает двадцатигранный кубик и возвращает значение от 1 до 20.
///
/// Реализация находится во вложенном приватном модуле, а эта функция
/// формирует публичный интерфейс модуля `engine`.
///
/// # Примеры
///
/// ```
/// let roll = modules_tests_docs::engine::dice20();
/// assert!((1..=20).contains(&roll));
/// ```
pub fn dice20() -> u8 {
    a::dice20()
}

#[cfg(test)]
mod tests {
    use super::dice20;

    #[test]
    fn dice20_is_greater_than_zero() {
        assert!(dice20() > 0);
    }

    #[test]
    fn dice20_is_less_than_21() {
        assert!(dice20() <= 20);
    }

    #[test]
    #[ignore = "проверка производительности миллиона бросков"]
    fn dice20_million_rolls() {
        for _ in 0..1_000_000 {
            assert!((1..=20).contains(&dice20()));
        }
    }
}
