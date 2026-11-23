use borrowing_references_slices::methods::Counter;
use std::rc::Rc;

fn main() {
    let mut counter = Counter::new(1);
    println!("&self: {}", counter.value());

    counter.increment();
    println!("После &mut self: {}", counter.value());
    println!("После self: {}", counter.into_inner());

    let shared = Rc::new(Counter::new(5));
    println!("self: &Rc<Self>: {}", shared.describe());
}
