/// Возвращает `"Lucky"` для чётного числа.
///
/// # Safety
///
/// `value` обязано быть чётным. Иначе функция вызывает неопределённое поведение.
unsafe fn even_or_unchecked(value: i32) -> &'static str {
    if value % 2 == 0 {
        "Lucky"
    } else {
        // SAFETY: контракт функции требует чётного value, поэтому эта ветка недостижима.
        unsafe { std::hint::unreachable_unchecked() }
    }
}

/// Безопасная обёртка, проверяющая контракт и завершающая работу при его нарушении.
pub fn even_from_i16(value: i16) -> &'static str {
    let value = i32::from(value);
    // SAFETY: весь диапазон i16 находится внутри i32, однако чётность всё ещё нужно проверить.
    if value % 2 != 0 {
        panic!("ожидалось чётное число");
    }
    unsafe { even_or_unchecked(value) }
}

/// Безопасная обёртка, проверяющая контракт во время выполнения.
pub fn even_checked(value: i32) -> Option<&'static str> {
    if value % 2 == 0 {
        // SAFETY: ветка выполняется только для чётного value.
        Some(unsafe { even_or_unchecked(value) })
    } else {
        None
    }
}

/// Возвращает элемент по индексу без повторной проверки границ.
pub fn checked_get<T>(slice: &[T], index: usize) -> Option<&T> {
    if index < slice.len() {
        // SAFETY: проверка выше гарантирует, что index находится в границах slice.
        Some(unsafe { slice.get_unchecked(index) })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_wrapper_handles_both_parity_cases() {
        assert_eq!(even_checked(8), Some("Lucky"));
        assert_eq!(even_checked(7), None);
    }

    #[test]
    fn checked_get_never_calls_unchecked_out_of_bounds() {
        let values = [10, 20, 30];
        assert_eq!(checked_get(&values, 1), Some(&20));
        assert_eq!(checked_get(&values, 3), None);
    }
}
