use modules_tests_docs::engine::{dice20, random_number};

#[test]
fn public_api_supports_a_game_turn() {
    let roll = dice20();
    let damage = random_number() + 3;

    assert!((1..=20).contains(&roll));
    assert_eq!(damage, 17);
}
