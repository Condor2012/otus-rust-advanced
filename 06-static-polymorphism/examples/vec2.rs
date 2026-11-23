use static_polymorphism::vec2::Vec2;

fn main() {
    let a = Vec2 { x: 1, y: 4 };
    let b = Vec2 { x: -9, y: 6 };
    println!("Сложение Vec2<i32>: {:?}", a + b);
    println!("Отрицание Vec2<i32>: {:?}", -a);

    let vector = Vec2 { x: 4.0_f32, y: 3.0 };
    println!("Длина Vec2<f32>: {}", vector.length());
}
