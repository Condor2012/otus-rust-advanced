use fp_oop::functional::{apply, compose, factorial, make_incrementer, map_rows, prepare_grid};

fn main() {
    println!("apply: {}", apply(|value| value * value, 5));
    let composed = compose(|value| value + 1, |value| value * 5);
    println!("compose: {}", composed(10));

    let mut grid = prepare_grid(3, 3);
    map_rows(&mut grid, |row| row[0] += 1);
    map_rows(&mut grid, make_incrementer(2));
    println!("Grid: {grid:?}");

    println!("factorial(5): {}", factorial(5));

    let first_five: Vec<_> = (0..).map(|value| value * value).take(5).collect();
    println!("Ленивый бесконечный итератор, take(5): {first_five:?}");
}
