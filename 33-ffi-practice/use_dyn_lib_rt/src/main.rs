type MyAddFn = unsafe extern "C" fn(left: u64, right: u64) -> u64;

fn main() {
    let path = "target/debug/libmy_lib.so";

    let mb_library = unsafe { libloading::Library::new(path) };
    let Ok(lib) = mb_library else {
        eprintln!("Failed to load library");
        return;
    };

    let mb_add_fn = unsafe { lib.get::<MyAddFn>("add") };
    let Ok(add_fn) = mb_add_fn else {
        eprintln!("Failed to load add fn");
        return;
    };

    let sum = unsafe { add_fn(1, 2) };
    println!("Hello, dyn rt sum: {sum}!");
}
