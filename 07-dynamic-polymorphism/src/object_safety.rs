//! Примеры из практики object safety.

pub trait Simple {
    fn by_ref(&self) {}
    fn by_ref_mut(&mut self) {}
}

pub trait ByValue {
    // Метод исключён из vtable, поэтому трейт остаётся dyn-compatible.
    fn by_value(self)
    where
        Self: Sized,
    {
    }

    fn by_ref(&self) {}
    fn by_ref_mut(&mut self) {}
}

pub trait Max {
    fn max(&self, other: &Self) -> bool {
        let _ = other;
        true
    }
}

pub trait MaxGeneric<Rhs> {
    fn max(&self, other: &Rhs) -> bool {
        let _ = other;
        true
    }
}

// Трейт с supertrait Clone не dyn-compatible, поскольку Clone возвращает Self:
// trait Foo: Clone {}

#[cfg(test)]
mod tests {
    use super::*;

    struct S;
    impl Simple for S {}
    impl ByValue for S {}
    impl Max for S {}
    impl MaxGeneric<Self> for S {}

    #[test]
    fn creates_supported_trait_objects() {
        let value = S;
        let simple: &dyn Simple = &value;
        let by_value: &dyn ByValue = &value;
        let max_generic: &dyn MaxGeneric<S> = &value;

        simple.by_ref();
        by_value.by_ref();
        // Max не dyn-compatible из-за параметра &Self, но работает с конкретным типом.
        assert!(Max::max(&value, &value));
        assert!(max_generic.max(&value));
    }
}
