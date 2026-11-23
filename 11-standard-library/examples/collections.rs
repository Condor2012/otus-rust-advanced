use standard_library::collections::{count_chars, ordered_unique_items, unique_chars};
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    let mut stack = Vec::with_capacity(4);
    stack.extend([1, 2, 3]);
    let popped = stack.pop();
    println!("Vec после pop: {stack:?}, извлечено: {popped:?}");

    let mut deque = VecDeque::from([2, 3]);
    deque.push_front(1);
    deque.push_back(4);
    println!("VecDeque: {deque:?}");

    let counts = count_chars("Let's count characters in this string");
    println!("Букв 't': {}", counts.get(&'t').copied().unwrap_or(0));
    println!(
        "Уникальных символов в abacaba: {}",
        unique_chars("abacaba").len()
    );
    println!(
        "Упорядоченные уникальные значения: {:?}",
        ordered_unique_items(vec![3, 1, 3, 2, 1])
    );

    let heap = BinaryHeap::from([3, 1, 4, 1, 5]);
    println!("Максимум BinaryHeap: {:?}", heap.peek());
}
