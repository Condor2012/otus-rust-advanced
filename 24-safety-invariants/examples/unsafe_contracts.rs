use safety_invariants::safety::{checked_get, even_checked, even_from_i16};

fn main() {
    println!("even_from_i16(42) = {}", even_from_i16(42));
    println!("even_checked(7) = {:?}", even_checked(7));
    println!("even_checked(8) = {:?}", even_checked(8));

    let values = [10, 20, 30];
    println!("checked_get(1) = {:?}", checked_get(&values, 1));
    println!("checked_get(3) = {:?}", checked_get(&values, 3));
}
