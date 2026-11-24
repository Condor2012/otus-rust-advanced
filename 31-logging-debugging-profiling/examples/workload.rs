use std::hint::black_box;

fn main() {
    let result = logging_debugging_profiling::sum_to(black_box(1_000_000));
    println!("Сумма: {result}");
}
