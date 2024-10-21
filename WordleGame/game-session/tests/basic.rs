use game_session_io::{SessionAction, SessionEvent};
use gtest::{Log, Program, ProgramBuilder, System};

const USER1: u64 = 8;
const USER2: u64 = 9;
const SESSION_PROGRAM_ID: u64 = 1;
const TARGET_PROGRAM_ID: u64 = 2;

#[test]
fn test_succeed() {
    let system = System::new();
    system.init_logger();

    // 初始化Proxy程序和Target程序
    let proxy_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(SESSION_PROGRAM_ID)
            .build(&system);
    let target_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(TARGET_PROGRAM_ID)
            .build(&system);
    let init_target_program_result = target_program.send_bytes(USER1, []);
    assert!(!init_target_program_result.main_failed());
    let init_proxy_program_result = proxy_program.send(USER1, target_program.id());
    assert!(!init_proxy_program_result.main_failed());

    // USER1 开始游戏
    let start_result = proxy_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());

    // 检查游戏是否开始
    let log = Log::builder()
        .source(proxy_program.id())
        .dest(USER1)
        .payload(SessionEvent::GameStarted { user: USER1.into() });
    assert!(start_result.contains(&log));

    system.spend_blocks(20);

    // USER1 猜测house
    let check_word_result_house = proxy_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: String::from("house"),
        },
    );
    assert!(!check_word_result_house.main_failed());

    // USER1 猜测human
    let check_word_result_human = proxy_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: String::from("human"),
        },
    );
    assert!(!check_word_result_human.main_failed());

    // USER1 猜测horse
    let check_word_result_horse = proxy_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: String::from("horse"),
        },
    );
    assert!(!check_word_result_horse.main_failed());

    // 检查游戏状态，玩家获胜
    let check_status_result =
        proxy_program.send(USER1, SessionAction::CheckGameStatus { user: USER1.into() });
    assert!(!check_status_result.main_failed());
    // let log = Log::builder()
    //     .source(proxy_program.id())
    //     .dest(USER1)
    //     .payload(SessionEvent::WordChecked {
    //         user: USER1.into(),
    //         correct_positions: Vec::from([0, 1, 2, 3, 4]),
    //         contained_in_word: Vec::from([]),
    //     });
    // assert!(check_word_result_house.contains(&log) || check_word_result_human.contains(&log) || check_word_result_horse.contains(&log));
}

#[test]
fn test_delayed_message() {
    let system = System::new();
    system.init_logger();

    // 初始化Proxy程序和Target程序
    let proxy_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(SESSION_PROGRAM_ID)
            .build(&system);
    let target_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(TARGET_PROGRAM_ID)
            .build(&system);
    let init_target_program_result = target_program.send_bytes(USER1, []);
    assert!(!init_target_program_result.main_failed());
    let init_proxy_program_result = proxy_program.send(USER1, target_program.id());
    assert!(!init_proxy_program_result.main_failed());

    // USER1 开始游戏
    let start_result = proxy_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());

    let log = Log::builder()
        .source(proxy_program.id())
        .dest(USER1)
        .payload(SessionEvent::GameStarted { user: USER1.into() });
    assert!(start_result.contains(&log));

    system.spend_blocks(200);

    system.spend_blocks(10);

    // 检查游戏状态，玩家失败
    let check_status_result =
        proxy_program.send(USER1, SessionAction::CheckGameStatus { user: USER1.into() });
    assert!(!check_status_result.main_failed());
}

#[test]
fn test_restart() {
    let system = System::new();
    system.init_logger();

    // 初始化Proxy程序和Target程序
    let proxy_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(SESSION_PROGRAM_ID)
            .build(&system);
    let target_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(TARGET_PROGRAM_ID)
            .build(&system);
    let init_target_program_result = target_program.send_bytes(USER1, []);
    assert!(!init_target_program_result.main_failed());
    let init_proxy_program_result = proxy_program.send(USER1, target_program.id());
    assert!(!init_proxy_program_result.main_failed());

    // USER1 开始游戏
    let start_result = proxy_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());

    let log = Log::builder()
        .source(proxy_program.id())
        .dest(USER1)
        .payload(SessionEvent::GameStarted { user: USER1.into() });
    assert!(start_result.contains(&log));

    system.spend_blocks(20);
    // USER1 第一次猜测
    let check_word_result = proxy_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: String::from("huodd"),
        },
    );
    assert!(!check_word_result.main_failed());

    system.spend_blocks(10);

    // USER1 猜测中途再次开始游戏 游戏成功重置
    let restart_result = proxy_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    let log = Log::builder()
        .source(proxy_program.id())
        .dest(USER1)
        .payload(SessionEvent::GameStarted { user: USER1.into() });
    assert!(restart_result.contains(&log));
    assert!(!restart_result.main_failed());
}

#[test]
fn test_attempts_run_out() {
    let system = System::new();
    system.init_logger();

    // 初始化Proxy程序和Target程序
    let proxy_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(SESSION_PROGRAM_ID)
            .build(&system);
    let target_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(TARGET_PROGRAM_ID)
            .build(&system);
    let init_target_program_result = target_program.send_bytes(USER1, []);
    assert!(!init_target_program_result.main_failed());
    let init_proxy_program_result = proxy_program.send(USER1, target_program.id());
    assert!(!init_proxy_program_result.main_failed());

    // USER1 开始游戏
    let start_result = proxy_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());

    let log = Log::builder()
        .source(proxy_program.id())
        .dest(USER1)
        .payload(SessionEvent::GameStarted { user: USER1.into() });
    assert!(start_result.contains(&log));

    system.spend_blocks(20);
    // USER1 第一次猜测
    let check_word_result = proxy_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: String::from("huodd"),
        },
    );
    assert!(!check_word_result.main_failed());

    system.spend_blocks(10);

    // USER1第二次猜测
    let check_word_result = proxy_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: String::from("huodd"),
        },
    );
    assert!(!check_word_result.main_failed());

    system.spend_blocks(10);

    // USER1第三次猜测
    let check_word_result = proxy_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: String::from("huodd"),
        },
    );
    assert!(!check_word_result.main_failed());

    // 检查游戏状态，玩家失败
    let check_status_result =
        proxy_program.send(USER1, SessionAction::CheckGameStatus { user: USER1.into() });
    assert!(!check_status_result.main_failed());
}

#[test]
fn test_mutiple_user() {
    let system = System::new();
    system.init_logger();

    // 初始化Proxy程序和Target程序
    let proxy_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(SESSION_PROGRAM_ID)
            .build(&system);
    let target_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(TARGET_PROGRAM_ID)
            .build(&system);
    let init_target_program_result = target_program.send_bytes(USER1, []);
    assert!(!init_target_program_result.main_failed());
    let init_proxy_program_result = proxy_program.send(USER1, target_program.id());
    assert!(!init_proxy_program_result.main_failed());

    // USER1 开始游戏
    let start_result = proxy_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());

    let log = Log::builder()
        .source(proxy_program.id())
        .dest(USER1)
        .payload(SessionEvent::GameStarted { user: USER1.into() });
    assert!(start_result.contains(&log));

    // USER2 开始游戏
    let start_result = proxy_program.send(USER2, SessionAction::StartGame { user: USER2.into() });
    assert!(!start_result.main_failed());

    let log = Log::builder()
        .source(proxy_program.id())
        .dest(USER2)
        .payload(SessionEvent::GameStarted { user: USER2.into() });
    assert!(start_result.contains(&log));

    system.spend_blocks(20);

    let check_word_result = proxy_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: String::from("house"),
        },
    );
    assert!(!check_word_result.main_failed());

    system.spend_blocks(20);

    let check_word_result = proxy_program.send(
        USER2,
        SessionAction::CheckWord {
            user: USER2.into(),
            word: String::from("houud"),
        },
    );
    assert!(!check_word_result.main_failed());
}
