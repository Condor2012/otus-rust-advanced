use static_polymorphism::shapes::{
    Circle, Rectangle, Triangle, compare_shapes, describe_verbose, get_area, print_shape,
};

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };
    let triangle = Triangle {
        base: 3.0,
        height: 8.0,
    };

    println!("{}", print_shape(&circle));
    println!("{}", print_shape(&rectangle));
    println!("{}", print_shape(&triangle));
    println!("Площадь круга: {:.2}", get_area(&circle));
    println!("{}", describe_verbose(&circle));
    println!("{}", compare_shapes(&circle, &rectangle));
}
