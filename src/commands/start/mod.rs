use actions::clear_terminal;
use clap::{Args, command};
use game::{GameMatrixWrapper, Player, gameloop};
use inquire::{Confirm, Select};
use strum::IntoEnumIterator;

use crate::{
    Failure, Output, ResultCode, ToolResult,
    services::{config::AppConfig, db::records::ToolsAnalytics, helper::HelperService},
};

mod game;
mod actions;
mod computer;
mod number_prompt;

#[derive(Debug, Args)]
#[command(about = ABOUT_START, long_about = LONG_ABOUT_START)]
pub struct ArgsStart {
    #[arg(short = 'p')]
    player: Option<Player>,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    multiplayer: bool,
    #[arg(short = 'd' , long, default_value="2", help="Game Difficulty; 1 = Easy, 2 = Normal, 3 = Hard")]
    difficulty: u8,
}

const ABOUT_START: &'static str = "Start the game";
const LONG_ABOUT_START: &'static str = "Start the TicTacToe Game";

pub fn handle(args: ArgsStart, cfg: &mut AppConfig, helper: &HelperService) -> ToolResult {
    let mut game_matrix: GameMatrixWrapper = GameMatrixWrapper::default();
    let mut player: Option<Player> = None;
    let multi_player: bool = args.multiplayer;

    if !multi_player {
        if args.player.as_ref().is_none() {
            let welcome_message = format!(
                "Welcome, {}. Please select a player: ",
                cfg.user.as_deref().unwrap_or("User")
            );
            match Select::new(&welcome_message, Player::iter().collect()).prompt() {
                Ok(g) => {
                    player = Some(g);
                }
                Err(e) => {
                    return Err(Failure {
                        message: "You have to select a player".to_string(),
                        trace: format!("Reason: {}", helper.generate_inquire_error(e)),
                        code: ResultCode::InvalidArgs,
                    });
                }
            }
        } else {
            player = Some(args.player.unwrap());
        }
    }

    if !multi_player && player.is_none() {
        return Err(Failure {
            message: "You have to select a player".to_string(),
            trace: format!(""),
            code: ResultCode::InvalidArgs,
        });
    }

    let _result = gameloop(&mut game_matrix, player, multi_player, args.difficulty, helper)?;

    match Confirm::new("Do you want to play again (Yes/No)? ").prompt() {
        Ok(g) => {
            match g {
                true => {
                    clear_terminal();
                    return handle(args, cfg, helper);
                },
                false => {},
            }
        },
        Err(_) => {},
    }

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
