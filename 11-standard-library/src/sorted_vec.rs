use std::ops::Deref;

/// Вектор, который сохраняет элементы отсортированными после каждой вставки.
#[derive(Debug, Clone)]
pub struct SortedVec<T> {
    inner: Vec<T>,
}

impl<T> Default for SortedVec<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value);
        self.inner.sort();
    }
}

impl<T> Deref for SortedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::SortedVec;

    #[test]
    fn keeps_values_sorted_and_supports_cloning() {
        let mut values = SortedVec::new();
        for value in [1, 500, 8, 3, 42] {
            values.push(value);
        }

        let mut cloned = values.clone();
        cloned.push(2);

        assert_eq!(values.as_slice(), &[1, 3, 8, 42, 500]);
        assert_eq!(cloned.as_slice(), &[1, 2, 3, 8, 42, 500]);
    }
}
