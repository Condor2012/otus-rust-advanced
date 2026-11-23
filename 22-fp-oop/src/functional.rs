pub fn apply<F, A, B>(function: F, argument: A) -> B
where
    F: Fn(A) -> B,
{
    function(argument)
}

pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |value| g(f(value))
}

pub type Grid = Vec<Vec<i32>>;

pub fn prepare_grid(rows: usize, columns: usize) -> Grid {
    vec![vec![0; columns]; rows]
}

pub fn map_rows<F>(grid: &mut Grid, function: F)
where
    F: Fn(&mut Vec<i32>),
{
    for row in grid {
        function(row);
    }
}

pub fn make_incrementer(amount: i32) -> impl Fn(&mut Vec<i32>) {
    move |row| row[0] += amount
}

pub const fn factorial(number: u64) -> u64 {
    match number {
        0 | 1 => 1,
        _ => number * factorial(number - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn higher_order_functions_accept_functions_and_closures() {
        assert_eq!(apply(|value| value * value, 5), 25);
        let increment_then_multiply = compose(|value| value + 1, |value| value * 5);
        assert_eq!(increment_then_multiply(10), 55);
    }

    #[test]
    fn higher_order_functions_transform_grid() {
        let mut grid = prepare_grid(3, 3);
        map_rows(&mut grid, |row| row[0] += 1);
        map_rows(&mut grid, make_incrementer(2));
        assert_eq!(grid, [[3, 0, 0], [3, 0, 0], [3, 0, 0]]);
    }

    #[test]
    fn factorial_is_recursive() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn iterators_are_lazy() {
        let evaluations = Cell::new(0);
        let mut numbers = (0..).inspect(|_| {
            evaluations.set(evaluations.get() + 1);
        });
        assert_eq!(evaluations.get(), 0);
        assert_eq!(numbers.next(), Some(0));
        assert_eq!(evaluations.get(), 1);
    }
}
