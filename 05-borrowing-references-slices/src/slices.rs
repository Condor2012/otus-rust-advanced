//! Представления последовательностей без передачи владения.

/// Возвращает сумму элементов любого слайса целых чисел.
pub fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

/// Возвращает средний элемент нечётного слайса или `None`.
pub fn middle(values: &[i32]) -> Option<&i32> {
    values.get(values.len() / 2)
}

#[cfg(test)]
mod tests {
    use super::{middle, sum};

    #[test]
    fn works_with_array_and_vector_slices() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        let vector = vec![4, 5];
        assert_eq!(sum(&vector), 9);
        assert_eq!(middle(&[1, 2, 3, 4, 5]), Some(&3));
        assert_eq!(middle(&[]), None);
    }
}
