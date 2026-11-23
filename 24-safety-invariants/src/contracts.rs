use std::num::NonZero;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Param1(NonZero<i32>);

impl Param1 {
    pub fn new(value: i32) -> Option<Self> {
        NonZero::new(value).map(Self)
    }

    pub fn get(self) -> i32 {
        self.0.get()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Param2(NonZero<i32>);

impl Param2 {
    pub fn new(value: i32) -> Option<Self> {
        NonZero::new(value).map(Self)
    }

    pub fn get(self) -> i32 {
        self.0.get()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Configured {
    param1: Param1,
    param2: Param2,
}

impl Configured {
    pub fn sum(self) -> i32 {
        self.param1.get() + self.param2.get()
    }
}

pub struct NeedsParam1;
pub struct NeedsParam2(Param1);

pub struct Builder<State> {
    state: State,
}

impl Builder<()> {
    pub fn new() -> Builder<NeedsParam1> {
        Builder { state: NeedsParam1 }
    }
}

impl Builder<NeedsParam1> {
    pub fn with_param1(self, param1: Param1) -> Builder<NeedsParam2> {
        Builder {
            state: NeedsParam2(param1),
        }
    }
}

impl Builder<NeedsParam2> {
    pub fn with_param2(self, param2: Param2) -> Configured {
        Configured {
            param1: self.state.0,
            param2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_enforces_initialization_order() {
        let configured = Builder::new()
            .with_param1(Param1::new(1).unwrap())
            .with_param2(Param2::new(2).unwrap());

        assert_eq!(configured.sum(), 3);
    }

    #[test]
    fn nonzero_parameters_reject_zero() {
        assert_eq!(Param1::new(0), None);
        assert_eq!(Param2::new(0), None);
    }
}
