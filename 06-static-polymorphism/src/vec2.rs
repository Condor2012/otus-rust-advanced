//! Решение практики с обобщённым двумерным вектором.

use std::ops::{Add, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Neg for Vec2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vec2<f32> {
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Vec2;

    #[test]
    fn supports_generic_add_and_neg() {
        assert_eq!(
            Vec2 { x: 1.0, y: 4.0 } + Vec2 { x: -9.0, y: 6.0 },
            Vec2 { x: -8.0, y: 10.0 }
        );
        assert_eq!(
            Vec2 { x: 1, y: 4 } + Vec2 { x: -9, y: 6 },
            Vec2 { x: -8, y: 10 }
        );
        assert_eq!(-Vec2 { x: 1, y: -4 }, Vec2 { x: -1, y: 4 });
    }

    #[test]
    fn calculates_f32_length() {
        assert_eq!(Vec2 { x: 4.0_f32, y: 3.0 }.length(), 5.0);
    }
}
