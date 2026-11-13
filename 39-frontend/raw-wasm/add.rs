// Голый wasm без wasm-bindgen: наружу видны только числа.
#![no_std]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Строку отдать нельзя — только указатель на неё в линейной памяти.
static HELLO: &[u8] = b"hello from linear memory";

#[unsafe(no_mangle)]
pub extern "C" fn hello_ptr() -> *const u8 {
    HELLO.as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn hello_len() -> usize {
    HELLO.len()
}
