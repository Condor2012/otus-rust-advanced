use borrowing_references_slices::slices::{middle, sum};

#[allow(
    clippy::useless_vec,
    reason = "Vec используется в исходном примере презентации"
)]
fn main() {
    let vector = vec![1, 2, 3, 4, 5];
    let all: &[i32] = &vector[..];
    let first = &vector[..1];
    let all_but_first = &vector[1..];
    let middle_slice = &vector[2..3];

    println!("Все элементы: {all:?}");
    println!("Первый: {first:?}");
    println!("Кроме первого: {all_but_first:?}");
    println!("Средний слайс: {middle_slice:?}");
    println!("Сумма: {}, средний элемент: {:?}", sum(all), middle(all));

    let text: &str = "Hello";
    let owned = String::from(text);
    println!("&str: {text}, String: {owned}");
}
