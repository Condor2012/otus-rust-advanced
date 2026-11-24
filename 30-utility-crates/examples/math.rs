use utility_crates::math::{
    camera_space_point, nalgebra_camera_space_point, rotate_with_glam, rotate_with_nalgebra,
    seeded_uniform_sample,
};

fn main() {
    println!("nalgebra, поворот: {}", rotate_with_nalgebra());
    println!("glam, поворот: {}", rotate_with_glam());
    println!("nalgebra, камера: {}", nalgebra_camera_space_point());
    println!("glam, камера: {}", camera_space_point());
    println!(
        "Воспроизводимая выборка: {:?}",
        seeded_uniform_sample(42, 8)
    );
}
