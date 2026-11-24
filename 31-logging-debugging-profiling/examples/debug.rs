use logging_debugging_profiling::checked_average;

fn main() {
    let values = vec![1, 2, 3];
    println!("Значения: {values:?}");
    dbg!(&values);
    dbg!(checked_average(&values));
    dbg!(checked_average(&[]));
}
