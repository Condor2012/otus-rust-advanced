use borrowing_references_slices::lifetimes::{Borrowed, first, longest};

fn main() {
    let first_string = String::from("Rust");
    let second_string = String::from("borrow");
    println!("longest: {}", longest(&first_string, &second_string));

    let selected = first(&first_string, &second_string);
    drop(second_string);
    println!("first остаётся действительной: {selected}");

    let number = 10;
    let borrowed = Borrowed { value: &number };
    println!("Заимствованное значение: {}", borrowed.value);
}
