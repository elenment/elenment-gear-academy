#![no_std]

use gtest::{Program, ProgramBuilder, System};
use game_session_io::*;

const USER1: u64 = 10;
const SESSION_PROGRAM_ID: u64 = 1;
const TARGET_PROGRAM_ID: u64 = 2;

#[test]
fn test_game_session_state() {
    let system = System::new();
    system.init_logger();

    let proxy_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
        .with_id(SESSION_PROGRAM_ID)
        .build(&system);

    let target_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
        .with_id(TARGET_PROGRAM_ID)
        .build(&system);

    let init_target_program_result = target_program.send_bytes(USER1, []);
    assert!(!init_target_program_result.main_failed());

    let init_proxy_program_result = proxy_program.send(USER1, target_program.id());
    assert!(!init_proxy_program_result.main_failed());

    let start_result = proxy_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());

     proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "house".into() });

     let state: Session = proxy_program.read_state(()).unwrap();
     assert_eq!(state.session_status, SessionStatus::Waiting);

     proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "horse".into() });
     proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "human".into() });

    let state: Session = proxy_program.read_state(()).unwrap();
    assert_eq!(state.session_status, SessionStatus::GameEnded { result: GameResult::Win });
}

#[test]
fn test_timeout() {
    let system = System::new();
    system.init_logger();

    let proxy_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
        .with_id(SESSION_PROGRAM_ID)
        .build(&system);

    let target_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
        .with_id(TARGET_PROGRAM_ID)
        .build(&system);

    let init_target_program_result = target_program.send_bytes(USER1, []);
    assert!(!init_target_program_result.main_failed());

    let init_proxy_program_result = proxy_program.send(USER1, target_program.id());
    assert!(!init_proxy_program_result.main_failed());

    let start_result = proxy_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());

    proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "hello".into() });

    system.spend_blocks(200);

    proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "hello".into() });
    assert!(!start_result.main_failed());

     let state: Session = proxy_program.read_state(()).unwrap();
     assert_eq!(state.session_status, SessionStatus::GameEnded { result: GameResult::Lose });
}
