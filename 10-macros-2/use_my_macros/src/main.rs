use my_macros::{SimpleDebug, log_call, say_hello};

fn main() {
    // say_hello_example();

    // my_function();

    let cat = Cat { name: "Whiskers".to_string(), age: 3 };
    println!("{cat:?}");

    let p = Point(1.0, 2.0);
    println!("{p:?}");

    let m = MyMarker;
    println!("{m:?}");
}

fn say_hello_example() {
    say_hello!("world");
    say_hello!(3 + 2 - 1);
    say_hello!(if 2 > 1 { "yes" } else { "no" });
}

#[log_call]
fn my_function() -> i32 {
    println!("My function body");
    42
}

#[derive(SimpleDebug)]
struct Cat {
    name: String,
    age: u32
}

#[derive(SimpleDebug)]
struct Point(f32, f32);

#[derive(SimpleDebug)]
struct MyMarker;
