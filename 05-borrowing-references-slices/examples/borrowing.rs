use borrowing_references_slices::borrowing::{replace_second, swap};

fn main() {
    let (mut x, mut y) = (5, 10);
    swap(&mut x, &mut y);
    println!("После swap: x = {x}, y = {y}");

    let mut values = vec![1, 2, 3];
    {
        let second = &values[1];
        println!("Разделяемая ссылка: {second}");
    }
    replace_second(&mut values, 10);
    println!("После изменения: {values:?}");
}
