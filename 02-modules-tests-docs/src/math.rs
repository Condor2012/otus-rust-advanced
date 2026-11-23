//! Математические операции учебного движка.

/// Возвращает детерминированное число для демонстрации работы модулей.
///
/// В настоящем игровом движке здесь мог бы находиться генератор случайных чисел.
/// Фиксированное значение делает пример воспроизводимым и не требует зависимостей.
///
/// # Примеры
///
/// ```
/// assert_eq!(modules_tests_docs::math::random_number(), 14);
/// ```
pub const fn random_number() -> u8 {
    14
}

#[cfg(test)]
mod tests {
    use super::random_number;

    #[test]
    fn returns_deterministic_number() {
        assert_eq!(random_number(), 14);
    }
}
