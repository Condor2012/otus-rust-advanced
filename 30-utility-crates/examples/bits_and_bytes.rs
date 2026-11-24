use bitvec::prelude::*;
use utility_crates::bits::{Color, Point, inline_numbers, points_as_bytes};

fn main() {
    let points = [Point(1.0, 2.0); 10];
    println!("Байт в срезе точек: {}", points_as_bytes(&points).len());

    let yellow = Color::GREEN | Color::BLUE;
    println!("GREEN | BLUE = {yellow:?}");
    println!("YELLOW ^ GREEN = {:?}", yellow ^ Color::GREEN);

    let mut bits = bitvec![u8, Msb0; 0, 0, 1, 0];
    bits.push(true);
    println!("Битовый вектор: {bits}");
    println!("Первые три бита: {}", &bits[..3]);

    let mut values = inline_numbers();
    values.push(3);
    println!("На стеке: {}; значения: {values:?}", !values.spilled());
    values.push(4);
    println!(
        "После пятого элемента используется куча: {}",
        values.spilled()
    );
}
