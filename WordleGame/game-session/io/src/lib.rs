#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, MessageId};
use wordle_io::*;

pub struct GameSessionMetadata;

impl Metadata for GameSessionMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<SessionAction, SessionEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<Session>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionAction {
    StartGame { user: ActorId },
    CheckWord { user: ActorId, word: String },
    CheckGameStatus { user: ActorId },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum GameResult {
    Win,
    Lose,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionEvent {
    GameStarted {
        user: ActorId,
    },
    WordChecked {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
    GameStatus(GameStatus),
    GameError(String),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionStatus {
    Waiting,
    MessageSent,
    MessageReceive(Event),
}

type SentMessageId = MessageId;
type OriginalMessageId = MessageId;

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Session {
    pub target_program_id: ActorId,
    pub msg_ids: (SentMessageId, OriginalMessageId),
    pub session_status: SessionStatus,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct WordGuessResult {
    pub word: String,
    pub correct_positions: Option<Vec<u8>>,
    pub contained_in_word: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct GameStatus {
    pub start_timestamp: u64,
    pub left_seconds: u64,
    pub left_attempts: u32,
    pub history: Vec<WordGuessResult>,
    pub game_result: Option<GameResult>,
}
