use polymorphism_practice::upcasting::{Dog, describe};

fn main() {
    let dog = Dog {
        name: "Барсик".to_string(),
        owner: "Юрий".to_string(),
    };
    println!("{}", describe(&dog));
}
