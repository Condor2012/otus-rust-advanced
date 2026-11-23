use dynamic_polymorphism::object_safety::{ByValue, Max, MaxGeneric, Simple};
use std::mem::size_of;

struct S;
impl Simple for S {}
impl ByValue for S {}
impl Max for S {}
impl MaxGeneric<Self> for S {}

fn main() {
    let value = S;
    let simple: &dyn Simple = &value;
    let by_value: &dyn ByValue = &value;
    let max_generic: &dyn MaxGeneric<S> = &value;

    simple.by_ref();
    by_value.by_ref();
    println!("Max для конкретного типа: {}", Max::max(&value, &value));
    println!("MaxGeneric: {}", max_generic.max(&value));
    println!("Размер &S: {} байт", size_of::<&S>());
    println!("Размер &dyn Simple: {} байт", size_of::<&dyn Simple>());
}
