#[cfg(feature = "real_rand")]
pub fn roll20() -> u32 {
    rand::random_range(1..=20)
}

#[cfg(not(feature = "real_rand"))]
pub fn roll20() -> u32 {
    20
}

