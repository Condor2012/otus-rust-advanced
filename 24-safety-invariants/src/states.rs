#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CharacterState {
    Fly { speed: f32, fuel: f32 },
    Fall { fall_time: f32 },
    Run { is_crouching: bool, speed: f32 },
}

impl CharacterState {
    pub fn speed(self) -> f32 {
        match self {
            Self::Fly { speed, .. } | Self::Run { speed, .. } => speed,
            Self::Fall { .. } => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_stores_only_relevant_data() {
        let flying = CharacterState::Fly {
            speed: 12.5,
            fuel: 80.0,
        };
        let falling = CharacterState::Fall { fall_time: 0.4 };

        assert_eq!(flying.speed(), 12.5);
        assert_eq!(falling.speed(), 0.0);
    }
}
