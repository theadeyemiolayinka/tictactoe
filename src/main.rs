use clap::{Parser, Subcommand, command};
use colored::Colorize;
use commands::start::ArgsStart;
use tictactoe::{
    commands::{self, init::{self, ArgsInit}, start}, services::{config::{codes::ResultCode, AppConfig}, crypt::CryptService, db::DBService, helper::HelperService}, Failure, Output, APP_NAME, CONFIG_NAME
};

#[derive(Parser)]
#[command()]
struct App {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    INIT(ArgsInit),
    START(ArgsStart),
}

fn main() {
    let app = App::parse();
    let mut cfg = load_app_config();
    let (crypt, db) = load_services();
    let helper = HelperService::new(crypt, db);

    let result: Result<Output, Failure>;

    if let Some(_) = &cfg.user {
        result = match app.command {
            Command::INIT(args) => init::handle(args, &mut cfg, &helper),
            Command::START(args) => start::handle(args, &mut cfg, &helper)
        };
    } else {
        result = match app.command {
            Command::INIT(args) => init::handle(args, &mut cfg, &helper),
            _ => Err(Failure { message: "You must initialize your details with the \"init\" command before continuing the game".to_string(), trace: "".to_string(), code: ResultCode::PCNameNotSet })
        };
    }

    match result {
        Ok(g) => _process_output(g),
        Err(e) => {
            let exit_code: i32 = e.code.as_i32();
            _process_failure(e);
            std::process::exit(exit_code);
        }
    }
}

fn load_app_config() -> AppConfig {
    let config = match confy::load(APP_NAME, CONFIG_NAME) {
        Ok(g) => Ok(g),
        Err(e) => Err(Failure {
            message: e.to_string(),
            trace: "".to_string(),
            code: ResultCode::ConfigUpdateFailed,
        }),
    };

    match config {
        Ok(config) => return config,
        Err(e) => {
            let exit_code = e.code.as_i32();
            _process_failure(e);
            std::process::exit(exit_code);
        }
    }
}

fn load_services() -> (CryptService, DBService) {
    let crypt = CryptService::new();
    if let Ok(crypt) = crypt {
        let db = DBService::new(Some(crypt.clone()));

        match db {
            Ok(db) => return (crypt, db),
            Err(e) => {
                let exit_code = e.code.as_i32();
                _process_failure(e);
                std::process::exit(exit_code);
            }
        }
    } else {
        let failure = crypt.err().unwrap();
        let exit_code = failure.code.as_i32();
        _process_failure(failure);
        std::process::exit(exit_code);
    }
}

fn _process_output(g: Output) {
    if let Some(message) = g.message {
        println!("{} {}", "✅ Success:".green().bold(), message.green());
    };
    if g.code.as_i32() > 0 {
        println!("Process completed with code: {}", g.code.as_i32());
    }
}

fn _process_failure(e: Failure) {
    println!("{} {}", "❌ Error:".red().bold(), e.message.red());
    if !e.trace.is_empty() {
        println!("{}", e.trace);
    }
    if e.code.as_i32() > 0 {
        println!("Process completed with code: {}", e.code.as_i32());
    }
}
