#![no_std]

use gstd::{exec, msg};
use pebbles_game_io::*;

// Define a static mutable variable to store the game state
static mut PEBBLES_GAME: Option<GameState> = None;

// Get a random 32-bit number
#[cfg(not(test))]
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

// Mock implementation for testing
#[cfg(test)]
fn get_random_u32() -> u32 {
    42 // Return a fixed number for testing purposes
}

// Find the best move strategy (in Hard mode)
fn find_best_move(pebbles_remaining: u32, max_pebbles_per_turn: u32) -> u32 {
    let mut best_move = 1;
    for i in 1..=max_pebbles_per_turn {
        if (pebbles_remaining - i) % (max_pebbles_per_turn + 1) == 0 {
            best_move = i;
            break;
        }
    }
    best_move
}

// Initialization function
#[no_mangle]
pub extern "C" fn init() {
    // Load initialization parameters
    let init: PebblesInit = msg::load().expect("Unable to load PebblesInit");

    // Check the validity of input data
    assert!(
        init.pebbles_count > 0,
        "Number of pebbles must be greater than 0"
    );
    assert!(
        init.max_pebbles_per_turn > 0,
        "Max pebbles per turn must be greater than 0"
    );

    // Randomly select the first player
    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    // Create game state
    let mut game_state = GameState {
        pebbles_count: init.pebbles_count,
        max_pebbles_per_turn: init.max_pebbles_per_turn,
        pebbles_remaining: init.pebbles_count,
        difficulty: init.difficulty.clone(),
        first_player: first_player.clone(),
        winner: None,
    };

    // If the first player is the program, the program makes the first move
    if let Player::Program = first_player {
        let pebbles_to_remove = match init.difficulty {
            DifficultyLevel::Easy => (get_random_u32() % init.max_pebbles_per_turn) + 1,
            DifficultyLevel::Hard => find_best_move(init.pebbles_count, init.max_pebbles_per_turn),
        };
        game_state.pebbles_remaining -= pebbles_to_remove;
        msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0).expect("Unable to reply");
    }

    // Save game state
    unsafe {
        PEBBLES_GAME = Some(game_state);
    }
}

// Handle function
#[no_mangle]
pub extern "C" fn handle() {
    // Load user action
    let action: PebblesAction = msg::load().expect("Unable to load PebblesAction");

    unsafe {
        if let Some(game_state) = PEBBLES_GAME.as_mut() {
            match action {
                PebblesAction::Turn(pebbles) => {
                    assert!(
                        pebbles > 0 && pebbles <= game_state.max_pebbles_per_turn,
                        "Invalid number of pebbles"
                    );

                    // User action
                    game_state.pebbles_remaining -= pebbles;
                    if game_state.pebbles_remaining == 0 {
                        game_state.winner = Some(Player::User);
                        msg::reply(PebblesEvent::Won(Player::User), 0).expect("Unable to reply");
                        return;
                    }

                    // Program action
                    let pebbles_to_remove = match game_state.difficulty {
                        DifficultyLevel::Easy => {
                            (get_random_u32() % game_state.max_pebbles_per_turn) + 1
                        }
                        DifficultyLevel::Hard => find_best_move(
                            game_state.pebbles_remaining,
                            game_state.max_pebbles_per_turn,
                        ),
                    };
                    game_state.pebbles_remaining -= pebbles_to_remove;
                    if game_state.pebbles_remaining == 0 {
                        game_state.winner = Some(Player::Program);
                        msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to reply");
                    } else {
                        msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0)
                            .expect("Unable to reply");
                    }
                }
                PebblesAction::GiveUp => {
                    game_state.winner = Some(Player::Program);
                    msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to reply");
                }
                PebblesAction::Restart {
                    difficulty,
                    pebbles_count,
                    max_pebbles_per_turn,
                } => {
                    assert!(
                        pebbles_count > 0,
                        "Number of pebbles must be greater than 0"
                    );
                    assert!(
                        max_pebbles_per_turn > 0,
                        "Max pebbles per turn must be greater than 0"
                    );

                    let first_player = if get_random_u32() % 2 == 0 {
                        Player::User
                    } else {
                        Player::Program
                    };

                    game_state.pebbles_count = pebbles_count;
                    game_state.max_pebbles_per_turn = max_pebbles_per_turn;
                    game_state.pebbles_remaining = pebbles_count;
                    game_state.difficulty = difficulty.clone();
                    game_state.first_player = first_player.clone();
                    game_state.winner = None;

                    if let Player::Program = first_player {
                        let pebbles_to_remove = match difficulty {
                            DifficultyLevel::Easy => (get_random_u32() % max_pebbles_per_turn) + 1,
                            DifficultyLevel::Hard => {
                                find_best_move(pebbles_count, max_pebbles_per_turn)
                            }
                        };
                        game_state.pebbles_remaining -= pebbles_to_remove;
                        msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0)
                            .expect("Unable to reply");
                    }
                }
            }
        }
    }
}

// State function
#[no_mangle]
extern "C" fn state() {
    unsafe {
        if let Some(game_state) = PEBBLES_GAME.as_ref() {
            msg::reply(game_state, 0).expect("Failed to share state");
        }
    }
}
