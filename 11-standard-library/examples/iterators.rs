use standard_library::words::Words;

fn main() {
    let input = [1, 2, 3];
    let parity = input.iter().map(|value| {
        print!("{value}");
        value % 2
    });
    println!("Итератор создан, но вычисления ещё не начались");
    for value in parity {
        print!("{value}");
    }
    println!();

    let first_five: Vec<_> = (0..).take(5).collect();
    let even_squares: Vec<_> = first_five
        .iter()
        .map(|value| value * value)
        .filter(|value| value % 2 == 0)
        .collect();
    println!("Первые пять: {first_five:?}");
    println!("Чётные квадраты: {even_squares:?}");

    let words = Words::new(vec!["Rust".into(), "standard".into(), "library".into()]);
    for (index, word) in words.iter().enumerate() {
        println!("{index}: {word}");
    }
    let joined = words.into_iter().collect::<Vec<_>>().join(" ");
    println!("IntoIterator: {joined}");
}
