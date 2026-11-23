use modules_tests_docs::engine::{dice20, random_number};

fn main() {
    let roll = dice20();
    println!("Ваш бросок: {roll}.");

    attack();
}

fn attack() {
    let damage = random_number() + 3;
    println!("Вы нанесли {damage} единиц урона.");
}
