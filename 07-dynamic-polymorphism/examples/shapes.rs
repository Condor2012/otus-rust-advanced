use dynamic_polymorphism::shapes::{
    Circle, Rectangle, Shape, Triangle, create_shape, largest, print_info, total_area,
};

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 4.0,
            height: 6.0,
        }),
        Box::new(Triangle {
            base: 3.0,
            height: 8.0,
        }),
    ];

    for shape in &shapes {
        println!("{}", print_info(shape.as_ref()));
    }
    println!("Суммарная площадь: {:.2}", total_area(&shapes));
    println!(
        "Самая большая: {}",
        largest(&shapes).expect("коллекция не пуста").name()
    );

    for kind in ["circle", "rectangle", "triangle"] {
        let shape = create_shape(kind, 3.0).expect("известный тип фигуры");
        println!("Фабрика: {}", shape.describe());
    }
}
