use clap::{Args, command};
use inquire::Text;

use crate::{
    Failure, Output, ResultCode, ToolResult,
    services::{config::AppConfig, db::records::ToolsAnalytics, helper::HelperService},
};

#[derive(Debug, Args)]
#[command(about = ABOUT_START, long_about = LONG_ABOUT_START)]
pub struct ArgsStart {
    #[arg(short = 'n', long = "name", help = "The PC's name")]
    pc_name: Option<String>,
}

const ABOUT_START: &'static str = "Start the game";
const LONG_ABOUT_START: &'static str = "Start the TicTacToe Game";

pub fn handle(args: ArgsStart, cfg: &mut AppConfig, helper: &HelperService) -> ToolResult {
    let analytics = helper.update_command_usage(ToolsAnalytics::START);

    Ok(Output {
        message: None,
        code: if analytics.is_ok() {
            ResultCode::Success
        } else {
            ResultCode::SuccessAnalyticsFailed
        },
    })
}
