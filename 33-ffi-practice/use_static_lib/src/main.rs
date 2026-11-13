#[link(name = "my_lib", kind = "static")]
unsafe extern "C" {
    fn add(left: u64, right: u64) -> u64;
}

fn main() {
    let sum = unsafe { add(1, 2) };
    println!("Hello, static sum: {sum}!");
}
