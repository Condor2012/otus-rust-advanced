//! Замыкания, указатели на функции и практика `apply`.

use std::ops::Add;

pub fn adder<T>(y: T) -> impl Fn(T) -> T + Copy
where
    T: Add<Output = T> + Copy,
{
    move |x| y + x
}

/// Применяет любой `FnOnce`, включая функции и замыкания.
pub fn apply<T, F>(f: F, x: T) -> T
where
    F: FnOnce(T) -> T,
{
    f(x)
}

/// Обобщённый `map` со статическим dispatch для замыкания.
pub fn map<T, U, F>(slice: &[T], func: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    slice.iter().map(func).collect()
}

/// Возвращает один из двух разных типов замыканий через трейт-объект.
pub fn returns_closure(triple: bool) -> Box<dyn Fn(i32) -> i32> {
    if triple {
        Box::new(|x| x * 3)
    } else {
        Box::new(|x| x)
    }
}

#[cfg(test)]
mod tests {
    use super::{adder, apply, map, returns_closure};

    #[test]
    #[allow(
        clippy::approx_constant,
        reason = "значения заданы в исходной практике"
    )]
    fn solves_apply_practice() {
        assert_eq!(apply(adder(5), 1), 6);
        assert_eq!(apply(adder(3.14), 8.0), 11.14);

        let add_one = adder(1);
        assert_eq!(apply(add_one, 5), 6);
        assert_eq!(apply(add_one, 6), 7);
    }

    #[test]
    fn supports_functions_and_closures() {
        fn triple(value: &i32) -> i32 {
            value * 3
        }

        let factor = 4;
        assert_eq!(map(&[1, 2, 3], triple), [3, 6, 9]);
        assert_eq!(map(&[1, 2, 3], |value| value * factor), [4, 8, 12]);
        assert_eq!(returns_closure(false)(5), 5);
        assert_eq!(returns_closure(true)(5), 15);
    }
}
