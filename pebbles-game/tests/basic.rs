use gstd::prelude::*;
use gtest::{Program, System};
use pebbles_game_io::*;

#[test]
fn test_init() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    let res = program.send_bytes(1, init_msg.encode());
    assert!(res.log().is_empty());

    let state: GameState = program.read_state().unwrap();
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 3);
    assert!(state.pebbles_remaining <= 15);
}

#[test]
fn test_user_turn() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(1, init_msg.encode());

    let action_msg = PebblesAction::Turn(2);
    let res = program.send_bytes(1, action_msg.encode());
    assert!(res.log().is_empty());

    let state: GameState = program.read_state().unwrap();
    assert_eq!(state.pebbles_remaining, 13);
}

#[test]
fn test_program_turn_easy() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(1, init_msg.encode());

    let action_msg = PebblesAction::Turn(2);
    let res = program.send_bytes(1, action_msg.encode());
    assert!(res.log().is_empty());

    let state: GameState = program.read_state().unwrap();
    assert!(state.pebbles_remaining <= 13);
}

#[test]
fn test_program_turn_hard() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(1, init_msg.encode());

    let action_msg = PebblesAction::Turn(2);
    let res = program.send_bytes(1, action_msg.encode());
    assert!(res.log().is_empty());

    let state: GameState = program.read_state().unwrap();
    assert!(state.pebbles_remaining <= 13);
}

#[test]
fn test_give_up() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(1, init_msg.encode());

    let action_msg = PebblesAction::GiveUp;
    let res = program.send_bytes(1, action_msg.encode());
    assert!(res.log().is_empty());

    let state: GameState = program.read_state().unwrap();
    assert_eq!(state.winner, Some(Player::Program));
}

#[test]
fn test_restart() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(1, init_msg.encode());

    let restart_msg = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 20,
        max_pebbles_per_turn: 4,
    };

    let res = program.send_bytes(1, restart_msg.encode());
    assert!(res.log().is_empty());

    let state: GameState = program.read_state().unwrap();
    assert_eq!(state.pebbles_count, 20);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert!(state.pebbles_remaining <= 20);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
}
