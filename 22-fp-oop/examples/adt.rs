use fp_oop::adt::{Color, List, Person, age_type, name_and_address_len, say_color};

fn main() {
    let list = List::Nil.prepend(3).prepend(2).prepend(1);
    println!(
        "Список: {list:?}, голова: {:?}, длина: {}",
        list.head(),
        list.len()
    );

    let person = Person {
        name: "Alice".into(),
        address: Some("Rust".into()),
    };
    println!("Person: {:?}", name_and_address_len(person));

    for age in [None, Some(-1), Some(18), Some(42), Some(151)] {
        println!("{age:?}: {}", age_type(age));
    }

    for color in [Color::Red, Color::Yellow, Color::Blue] {
        println!("Color: {}", say_color(&color));
    }
}
