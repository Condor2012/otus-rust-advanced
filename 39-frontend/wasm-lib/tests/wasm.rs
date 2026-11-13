//! Тесты, которые выполняются ВНУТРИ wasm (node через wasm-bindgen-test-runner).
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn sum_primes_large() {
    // Известное значение: сумма простых до 20 000 000
    assert_eq!(wasm_lib::sum_primes(20_000_000), 12_272_577_818_052);
}

#[wasm_bindgen_test]
fn usize_is_32_bit_here() {
    assert_eq!(usize::BITS, 32);
}
