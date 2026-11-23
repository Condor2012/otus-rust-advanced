//! Гетерогенная коллекция фигур через `Box<dyn Shape>`.

use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;

    fn describe(&self) -> String {
        format!("[{}] area = {:.2}", self.name(), self.area())
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

    fn name(&self) -> &str {
        "Circle"
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn name(&self) -> &str {
        "Triangle"
    }
}

pub fn print_info(shape: &dyn Shape) -> String {
    shape.describe()
}

/// Создаёт фигуру по имени, полученному во время выполнения.
pub fn create_shape(kind: &str, size: f64) -> Option<Box<dyn Shape>> {
    match kind {
        "circle" => Some(Box::new(Circle { radius: size })),
        "rectangle" => Some(Box::new(Rectangle {
            width: size,
            height: size,
        })),
        "triangle" => Some(Box::new(Triangle {
            base: size,
            height: size,
        })),
        _ => None,
    }
}

pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

pub fn largest(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    shapes.iter().map(Box::as_ref).max_by(|a, b| {
        a.area()
            .partial_cmp(&b.area())
            .expect("площадь фигуры не должна быть NaN")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shapes() -> Vec<Box<dyn Shape>> {
        vec![
            Box::new(Circle { radius: 5.0 }),
            Box::new(Rectangle {
                width: 4.0,
                height: 6.0,
            }),
            Box::new(Triangle {
                base: 3.0,
                height: 8.0,
            }),
        ]
    }

    #[test]
    fn supports_heterogeneous_collection() {
        let shapes = shapes();

        assert_eq!(shapes[2].name(), "Triangle");
        assert!((total_area(&shapes) - (25.0 * PI + 36.0)).abs() < 1e-10);
        assert_eq!(
            largest(&shapes).expect("коллекция не пуста").name(),
            "Circle"
        );
    }

    #[test]
    fn factory_selects_type_at_runtime() {
        assert_eq!(
            create_shape("triangle", 3.0).expect("known kind").area(),
            4.5
        );
        assert!(create_shape("ellipse", 3.0).is_none());
    }
}
