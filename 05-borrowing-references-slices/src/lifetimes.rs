//! Явные времена жизни и правила их вывода.

/// Возвращает более длинную из двух строк.
///
/// Время жизни результата ограничено временем жизни обоих аргументов.
pub fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

/// Возвращает первую строку; вторая не участвует во времени жизни результата.
pub fn first<'a>(value: &'a str, _other: &str) -> &'a str {
    value
}

/// Структура, хранящая заимствованное целое число.
pub struct Borrowed<'a> {
    pub value: &'a i32,
}

#[cfg(test)]
mod tests {
    use super::{Borrowed, first, longest};

    #[test]
    fn returned_references_remain_connected_to_inputs() {
        let number = 10;
        let borrowed = Borrowed { value: &number };

        assert_eq!(longest("Rust", "borrow"), "borrow");
        assert_eq!(first("first", "second"), "first");
        assert_eq!(*borrowed.value, 10);
    }
}
