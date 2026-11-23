use standard_library::sorted_vec::SortedVec;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let mut boxed = Box::new(Point { x: 1.0, y: 2.0 });
    let original = Point { x: 1.0, y: 2.0 };
    assert_eq!(*boxed, original);
    boxed.x = 5.0;
    assert_ne!(*boxed, original);
    println!("Box после изменения: {boxed:?}");

    let boxed_string = Box::new(String::from("Deref"));
    println!("Длина Box<String>: {}", boxed_string.len());

    let mut map = HashMap::new();
    let first = {
        let value: &i32 = map.entry(5).or_insert(10);
        *value
    };
    let second = {
        let value: &i32 = map.entry(10).or_insert(20);
        *value
    };
    println!("Первое: {first}, второе: {second}");

    let cell = Cell::new(10);
    cell.set(5);
    let previous = cell.replace(0);
    println!("Cell: предыдущее = {previous}, текущее = {}", cell.get());

    let state = Rc::new(RefCell::new(Point { x: 1.0, y: 2.0 }));
    let shared = Rc::clone(&state);
    state.borrow_mut().x = 10.0;
    println!(
        "Rc<RefCell<T>>: x = {}, владельцев = {}",
        shared.borrow().x,
        Rc::strong_count(&shared)
    );

    let mut sorted = SortedVec::new();
    for value in [1, 500, 8, 3, 42] {
        sorted.push(value);
    }
    let mut cloned = sorted.clone();
    cloned.push(2);
    println!("SortedVec: {:?}", sorted.as_slice());
    println!("Клон после вставки: {:?}", cloned.as_slice());
}
