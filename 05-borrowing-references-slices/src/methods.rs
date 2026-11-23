//! Варианты получателя `self` в методах.

use std::rc::Rc;

#[derive(Default)]
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    /// Читает значение через разделяемую ссылку.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Изменяет значение через уникальную ссылку.
    pub fn increment(&mut self) {
        self.value += 1;
    }

    /// Забирает владение объектом и возвращает внутреннее значение.
    pub fn into_inner(self) -> i32 {
        self.value
    }

    /// Получатель может быть ссылкой на `Rc<Self>`.
    pub fn describe(self: &Rc<Self>) -> String {
        format!("Rc<Counter>({})", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;
    use std::rc::Rc;

    #[test]
    fn supports_all_receiver_forms() {
        let mut counter = Counter::new(1);
        assert_eq!(counter.value(), 1);
        counter.increment();
        assert_eq!(counter.into_inner(), 2);

        let shared = Rc::new(Counter::new(5));
        assert_eq!(shared.describe(), "Rc<Counter>(5)");
    }
}
