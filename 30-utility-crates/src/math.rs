use glam::{Mat4, Vec3, Vec4};
use nalgebra::{Point3, Point4, Rotation2, Vector2, Vector3};
use rand::distr::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn rotate_with_nalgebra() -> Vector2<f32> {
    let point = Vector2::new(1.0, 1.0);
    Rotation2::new(std::f32::consts::FRAC_PI_2) * point
}

pub fn rotate_with_glam() -> glam::Vec2 {
    let point = glam::Vec2::new(1.0, 1.0);
    glam::Mat2::from_angle(std::f32::consts::FRAC_PI_2) * point
}

pub fn camera_space_point() -> Vec4 {
    let point = Vec4::new(0.0, 0.0, 0.0, 1.0);
    let position = Vec3::new(0.5, 0.5, -1.0);
    let target = Vec3::new(0.5, 0.5, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    Mat4::look_at_lh(position, target, up) * point
}

pub fn nalgebra_camera_space_point() -> Point4<f32> {
    let point = Point4::new(0.0, 0.0, 0.0, 1.0);
    let position = Point3::new(0.5, 0.5, -1.0);
    let target = Point3::new(0.5, 0.5, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    nalgebra::Matrix4::look_at_lh(&position, &target, &up) * point
}

pub fn seeded_uniform_sample(seed: u64, count: usize) -> Vec<u32> {
    let mut rng = StdRng::seed_from_u64(seed);
    let distribution = Uniform::new(0, 8).expect("диапазон не пуст");
    (0..count).map(|_| distribution.sample(&mut rng)).collect()
}

pub fn seeded_range(seed: u64, count: usize, range: std::ops::Range<u32>) -> Vec<u32> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..count)
        .map(|_| rng.random_range(range.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotations_agree() {
        let a = rotate_with_nalgebra();
        let b = rotate_with_glam();
        assert!((a.x - b.x).abs() < 1e-5);
        assert!((a.y - b.y).abs() < 1e-5);
    }

    #[test]
    fn seeded_sequence_is_reproducible() {
        assert_eq!(seeded_uniform_sample(42, 16), seeded_uniform_sample(42, 16));
        assert!(seeded_range(7, 16, 0..3).iter().all(|value| *value < 3));
    }
}
