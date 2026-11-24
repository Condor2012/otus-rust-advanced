use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use smallvec::{SmallVec, smallvec};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Point(pub f32, pub f32);

pub fn points_as_bytes(points: &[Point]) -> &[u8] {
    bytemuck::cast_slice(points)
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Color: u8 {
        const RED = 0b0000_0001;
        const GREEN = 0b0000_0010;
        const BLUE = 0b0000_0100;
        const YELLOW = Self::GREEN.bits() | Self::BLUE.bits();
        const WHITE = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits();
    }
}

pub fn inline_numbers() -> SmallVec<[i32; 4]> {
    smallvec![0, 1, 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pod_points_have_stable_byte_representation() {
        let points = [Point(1.0, 2.0), Point(3.0, 4.0)];
        assert_eq!(points_as_bytes(&points).len(), 4 * size_of::<f32>());
        let restored: &[Point] = bytemuck::cast_slice(points_as_bytes(&points));
        assert_eq!(restored[1].1, 4.0);
    }

    #[test]
    fn flags_support_set_operations() {
        let green = Color::GREEN;
        let yellow = green | Color::BLUE;
        assert_eq!(yellow, Color::YELLOW);
        assert_eq!(yellow & green, Color::GREEN);
        assert_eq!(yellow ^ green, Color::BLUE);
    }

    #[test]
    fn smallvec_spills_only_after_inline_capacity() {
        let mut values = inline_numbers();
        assert!(!values.spilled());
        values.push(3);
        assert!(!values.spilled());
        values.push(4);
        assert!(values.spilled());
    }
}
