use polymorphism_practice::closures::{adder, apply, map, returns_closure};

fn main() {
    println!("apply(adder(5), 1) = {}", apply(adder(5), 1));

    let add_one = adder(1);
    println!("Повторное apply: {}", apply(add_one, 5));
    println!("Повторное apply: {}", apply(add_one, 6));

    let factor = 3;
    println!("map с захватом: {:?}", map(&[1, 2, 3, 4], |x| x * factor));
    println!("Box<dyn Fn>: {}", returns_closure(true)(5));
}
