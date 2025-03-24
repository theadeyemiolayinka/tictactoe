use serde::{Deserialize, Serialize};
use services::config::codes::ResultCode;

pub mod services;
pub mod commands;

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub message: Option<String>,
    pub code: ResultCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Failure {
    pub message: String,
    pub trace: String,
    pub code: ResultCode,
}

pub type AppResult<T> = Result<T, Failure>;
type ToolResult = AppResult<Output>;

pub static APP_AUTHOR: &'static str = "TheAdeyemiOlayinka";
pub static APP_ABOUT: &'static str =
    "A TicTacToe Game Implementation in Rust.";
pub static DEFAULT_USER: &'static str = "TheAdeyemiOlayinka";
pub static APP_NAME: &'static str = if cfg!(target_os = "windows") {
    "TheAdeyemiOlayinka/tictactoe-rs"
} else {
    "theadeyemiolayinka/tictactoe-rs"
};
pub static CONFIG_NAME: &'static str = "tictactoe-rs";
