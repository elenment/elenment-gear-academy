#![no_std]

use gstd::{msg, exec};
use pebbles_game_io::*;

// 定义静态可变变量，用于存储游戏状态
static mut PEBBLES_GAME: Option<GameState> = None;

// 获取一个随机的32位数
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

// 找到最佳移动策略（Hard模式下）
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

// 初始化函数
#[no_mangle]
pub extern "C" fn init() {
    // 加载初始化参数
    let init: PebblesInit = msg::load().expect("Unable to load PebblesInit");

    // 检查输入数据的有效性
    assert!(init.pebbles_count > 0, "Number of pebbles must be greater than 0");
    assert!(init.max_pebbles_per_turn > 0, "Max pebbles per turn must be greater than 0");

    // 随机选择第一个玩家
    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    // 创建游戏状态
    let mut game_state = GameState {
        pebbles_count: init.pebbles_count,
        max_pebbles_per_turn: init.max_pebbles_per_turn,
        pebbles_remaining: init.pebbles_count,
        difficulty: init.difficulty.clone(),
        first_player: first_player.clone(),
        winner: None,
    };

    // 如果第一个玩家是程序，则程序进行第一次操作
    if let Player::Program = first_player {
        let pebbles_to_remove = match init.difficulty {
            DifficultyLevel::Easy => (get_random_u32() % init.max_pebbles_per_turn) + 1,
            DifficultyLevel::Hard => find_best_move(init.pebbles_count, init.max_pebbles_per_turn),
        };
        game_state.pebbles_remaining -= pebbles_to_remove;
        msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0).expect("Unable to reply");
    }

    // 保存游戏状态
    unsafe {
        PEBBLES_GAME = Some(game_state);
    }
}

// 处理函数
#[no_mangle]
pub extern "C" fn handle() {
    // 加载用户操作
    let action: PebblesAction = msg::load().expect("Unable to load PebblesAction");

    unsafe {
        if let Some(game_state) = PEBBLES_GAME.as_mut() {
            match action {
                PebblesAction::Turn(pebbles) => {
                    assert!(pebbles > 0 && pebbles <= game_state.max_pebbles_per_turn, "Invalid number of pebbles");

                    // 用户操作
                    game_state.pebbles_remaining -= pebbles;
                    if game_state.pebbles_remaining == 0 {
                        game_state.winner = Some(Player::User);
                        msg::reply(PebblesEvent::Won(Player::User), 0).expect("Unable to reply");
                        return;
                    }

                    // 程序操作
                    let pebbles_to_remove = match game_state.difficulty {
                        DifficultyLevel::Easy => (get_random_u32() % game_state.max_pebbles_per_turn) + 1,
                        DifficultyLevel::Hard => find_best_move(game_state.pebbles_remaining, game_state.max_pebbles_per_turn),
                    };
                    game_state.pebbles_remaining -= pebbles_to_remove;
                    if game_state.pebbles_remaining == 0 {
                        game_state.winner = Some(Player::Program);
                        msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to reply");
                    } else {
                        msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0).expect("Unable to reply");
                    }
                },
                PebblesAction::GiveUp => {
                    game_state.winner = Some(Player::Program);
                    msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to reply");
                },
                PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
                    assert!(pebbles_count > 0, "Number of pebbles must be greater than 0");
                    assert!(max_pebbles_per_turn > 0, "Max pebbles per turn must be greater than 0");

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
                            DifficultyLevel::Hard => find_best_move(pebbles_count, max_pebbles_per_turn),
                        };
                        game_state.pebbles_remaining -= pebbles_to_remove;
                        msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0).expect("Unable to reply");
                    }
                }
            }
        }
    }
}

// 状态函数
#[no_mangle]
extern "C" fn state() {
    unsafe {
        if let Some(game_state) = PEBBLES_GAME.as_ref() {
            msg::reply(game_state, 0).expect("Failed to share state");
        }
    }
}
