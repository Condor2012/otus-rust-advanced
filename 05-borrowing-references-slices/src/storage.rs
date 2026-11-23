//! Решение практики по ссылке из презентации.
//!
//! В исходном задании `push` и `get` не связывали ссылки с временем жизни `Storage`.
//! Явные типы `&'a T` в обеих сигнатурах исправляют эти ошибки.

pub struct Storage<'a, T> {
    inner: Vec<&'a T>,
}

impl<'a, T> Storage<'a, T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.inner.get(index).copied()
    }

    pub fn push(&mut self, value: &'a T) {
        self.inner.push(value);
    }
}

impl<T> Default for Storage<'_, T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Storage;

    #[test]
    fn returns_reference_after_storage_is_dropped() {
        let (x, y) = (10, 20);
        let ref_x = {
            let mut storage = Storage::<i32>::new();
            storage.push(&x);
            storage.push(&y);
            storage.get(0).expect("первый элемент должен существовать")
        };

        assert_eq!(*ref_x, 10);
    }
}
