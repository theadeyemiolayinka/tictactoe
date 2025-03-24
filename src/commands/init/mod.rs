use clap::{command, Args};
use inquire::Text;

use crate::{
    services::{
        config::AppConfig, db::records::ToolsAnalytics, helper::HelperService
    },
    Failure, Output, ResultCode, ToolResult,
};

#[derive(Debug, Args)]
#[command(about = ABOUT_INIT, long_about = LONG_ABOUT_INIT)]
pub struct ArgsInit {
    #[arg(short = 'n', long = "name", help = "Your name")]
    name: Option<String>,
}

const ABOUT_INIT: &'static str = "Initialize toolkit";
const LONG_ABOUT_INIT: &'static str = "Initialize the toolkit by setting a name.";

pub fn handle(args: ArgsInit, cfg: &mut AppConfig, helper: &HelperService) -> ToolResult {
    let name: String;

    if args
        .name
        .as_ref()
        .is_none_or(|g| g.is_empty() || g == "")
    {
        match Text::new("Enter Your Name:").prompt() {
            Ok(g) => {
                if g.is_empty() {
                    return Err(Failure {
                        message: "Name can not be empty".to_string(),
                        trace: "".to_string(),
                        code: ResultCode::InvalidArgs,
                    });
                }
                name = g
            }
            Err(e) => {
                return Err(Failure {
                    message: "Name can not be empty".to_string(),
                    trace: format!(
                        "Reason: {}",
                        match e {
                            inquire::InquireError::NotTTY =>
                                "Input device is not a TTY".to_string(),
                            inquire::InquireError::InvalidConfiguration(reason) =>
                                format!("Invalid configuration: {}", reason),
                            inquire::InquireError::IO(io_err) => format!("IO error: {}", io_err),
                            inquire::InquireError::OperationCanceled =>
                                "Operation canceled by the user".to_string(),
                            inquire::InquireError::OperationInterrupted =>
                                "Operation interrupted by the user".to_string(),
                            inquire::InquireError::Custom(custom_err) =>
                                format!("Custom error: {}", custom_err),
                        }
                    ),
                    code: ResultCode::InvalidArgs,
                })
            }
        }
    } else {
        name = args.name.clone().unwrap();
    }

    cfg.user = Some(name);

    match cfg.update() {
        Ok(_) => {
            let analytics = helper.update_command_usage(ToolsAnalytics::INIT);

            Ok(Output {
                message: Some("Name Set Successfully!".to_string()),
                code: if analytics.is_ok() { ResultCode::Success } else { ResultCode::SuccessAnalyticsFailed },
            })
        }
        Err(e) => Err(e),
    }
}
