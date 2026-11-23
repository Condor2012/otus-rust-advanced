//! Внутренняя реализация игрового движка.

pub(super) fn dice20() -> u8 {
    b::dice20()
}

mod b {
    pub(super) fn dice20() -> u8 {
        crate::math::random_number()
    }
}
