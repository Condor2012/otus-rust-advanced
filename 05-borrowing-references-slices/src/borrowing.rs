//! Разделяемые и уникальные ссылки.

/// Меняет два значения местами через уникальные ссылки.
///
/// Компилятор гарантирует, что `x` и `y` не ссылаются на одно значение.
#[allow(
    clippy::manual_swap,
    reason = "ручной обмен показан в презентации занятия"
)]
pub fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

/// Заменяет второй элемент слайса через уникальную ссылку.
pub fn replace_second(values: &mut [i32], value: i32) {
    values[1] = value;
}

#[cfg(test)]
mod tests {
    use super::{replace_second, swap};

    #[test]
    fn swaps_distinct_values() {
        let (mut x, mut y) = (5, 10);
        swap(&mut x, &mut y);
        assert_eq!((x, y), (10, 5));
    }

    #[test]
    fn mutates_slice_through_unique_reference() {
        let mut values = [1, 2, 3];
        replace_second(&mut values, 10);
        assert_eq!(values, [1, 10, 3]);
    }
}
