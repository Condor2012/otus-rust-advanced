use safety_invariants::states::CharacterState;

fn main() {
    let states = [
        CharacterState::Fly {
            speed: 12.5,
            fuel: 80.0,
        },
        CharacterState::Fall { fall_time: 0.4 },
        CharacterState::Run {
            is_crouching: true,
            speed: 2.0,
        },
    ];

    for state in states {
        println!("{state:?}: speed = {}", state.speed());
    }
}
