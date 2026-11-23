//! Геометрические фигуры из лайфкодинга и практики занятия.

use std::f64::consts::PI;
use std::fmt::{Display, Formatter};

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &str;

    fn describe(&self) -> String {
        format!(
            "[{}] area={:.2} perimeter={:.2}",
            self.name(),
            self.area(),
            self.perimeter()
        )
    }
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn perimeter(&self) -> f64 {
        // Для учебного треугольника из практики известны основание и высота.
        // Считаем его прямоугольным, чтобы вычислить гипотенузу.
        self.base + self.height + self.base.hypot(self.height)
    }

    fn name(&self) -> &str {
        "Triangle"
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle(r={})", self.radius)
    }
}

pub fn print_shape<T: Shape>(shape: &T) -> String {
    shape.describe()
}

pub fn get_area<T: Shape>(shape: &T) -> f64 {
    shape.area()
}

pub fn describe_verbose<T: Shape + Display>(shape: &T) -> String {
    format!("Display: {shape}\n{}", shape.describe())
}

pub fn compare_shapes<A, B>(first: &A, second: &B) -> String
where
    A: Shape,
    B: Shape,
{
    if first.area() > second.area() {
        format!("{} больше", first.name())
    } else {
        format!("{} больше", second.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_shape_properties() {
        let circle = Circle { radius: 5.0 };
        let rectangle = Rectangle {
            width: 4.0,
            height: 6.0,
        };
        let triangle = Triangle {
            base: 3.0,
            height: 8.0,
        };

        assert!((circle.area() - 25.0 * PI).abs() < f64::EPSILON);
        assert_eq!(rectangle.area(), 24.0);
        assert_eq!(rectangle.perimeter(), 20.0);
        assert_eq!(triangle.area(), 12.0);
        assert_eq!(
            print_shape(&rectangle),
            "[Rectangle] area=24.00 perimeter=20.00"
        );
        assert_eq!(compare_shapes(&circle, &rectangle), "Circle больше");
        assert!(describe_verbose(&circle).starts_with("Display: Circle(r=5)"));
    }
}
