use clap::ValueEnum;
use colored::Colorize;
use strum_macros::{Display, EnumIter};

use crate::{
    AppResult, Failure,
    services::{config::codes::ResultCode, helper::HelperService},
};

use super::{
    actions::{
        check_win, clear_terminal, print_final_game_matrix, print_selectable_game_matrix,
        select_position,
    }, computer::make_move, number_prompt::NumberPrompt
};

pub type GameMatrix = [[i32; 3]; 3];
pub type WinData = (Option<Player>, Vec<(usize, usize)>);

pub struct GameMatrixWrapper(pub GameMatrix);

impl GameMatrixWrapper {
    pub fn value(&self) -> GameMatrix {
        self.0
    }

    pub fn set_position(&mut self, pos: (usize, usize), value: Player) {
        self.0[pos.0][pos.1] = value.as_i32();
    }
}

impl Default for GameMatrixWrapper {
    fn default() -> Self {
        GameMatrixWrapper([[0; 3]; 3])
    }
}

#[derive(ValueEnum, EnumIter, Display, Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X = 1,
    O = 2,
}

impl Player {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            x if x == Player::X.as_i32() => Some(Player::X),
            x if x == Player::O.as_i32() => Some(Player::O),
            _ => None,
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

pub struct PlayerTurn {
    player: Player,
    game_complete: bool,
    win_data: WinData,
}

impl PlayerTurn {
    pub fn play(
        &mut self,
        mut game_matrix: &mut GameMatrixWrapper,
        _player: Option<Player>,
        multi_player: bool,
        difficulty: u8,
        helper: &HelperService,
    ) -> AppResult<()> {
        let available_cells = print_selectable_game_matrix(game_matrix);
        if available_cells <= 0 {
            self.game_complete = true;
            return Ok(());
        }
        if multi_player {
            println!("");
            let player_turn_message = match self.player {
                Player::X => format!("{} Turn", self.player.to_string()).red().bold(),
                Player::O => format!("{} Turn", self.player.to_string()).blue().bold(),
            };
            let position = NumberPrompt::prompt(
                format!("{}: Select a position to play: >", player_turn_message).as_str(),
            )
            .map_err(|e| Failure {
                message: "Invalid input".to_string(),
                trace: format!("Reason: {}", helper.generate_inquire_error(e)),
                code: ResultCode::CancelOperation,
            })?;

            let res = select_position(game_matrix, position as u16, self.player);
            self.check_winner(game_matrix);
            res
        } else {
            if _player.is_some_and(|g| g == self.player) {
                println!("");
                let position =
                    NumberPrompt::prompt(format!("Select a position to play: >").as_str())
                        .map_err(|e| Failure {
                            message: "Invalid input".to_string(),
                            trace: format!("Reason: {}", helper.generate_inquire_error(e)),
                            code: ResultCode::CancelOperation,
                        })?;

                let res = select_position(game_matrix, position as u16, self.player);
                self.check_winner(game_matrix);
                res
            } else {
                make_move(&mut game_matrix, self.player, difficulty);
                self.check_winner(game_matrix);
                Ok(())
            }
        }
    }

    pub fn change_turns(&mut self) {
        match self.player {
            Player::X => {
                self.player = Player::O;
            }
            Player::O => {
                self.player = Player::X;
            }
        }
    }

    pub fn check_winner(&mut self, game_matrix: &mut GameMatrixWrapper) {
        let res = check_win(game_matrix).unwrap();
        if res.0.is_some() {
            self.game_complete = true;
        }
        self.win_data = res;
    }
}

impl Default for PlayerTurn {
    fn default() -> Self {
        Self {
            player: Player::X,
            game_complete: false,
            win_data: (None, vec![]),
        }
    }
}

pub fn gameloop(
    game_matrix: &mut GameMatrixWrapper,
    player: Option<Player>,
    multi_player: bool,
    difficulty: u8,
    helper: &HelperService,
) -> AppResult<()> {
    let mut turn = PlayerTurn::default();
    clear_terminal();
    loop {
        let res = turn.play(game_matrix, player, multi_player, difficulty, helper);
        match res {
            Ok(_) => {
                turn.change_turns();
                clear_terminal();
            }
            Err(e) => {
                println!("{}: {}\n", e.message, e.trace);
                if e.code == ResultCode::CancelOperation {
                    return Err(e);
                }
            }
        }
        if turn.game_complete {
            turn.check_winner(game_matrix);

            print_final_game_matrix(game_matrix, turn.win_data.1);

            if turn.win_data.0.is_none() {
                println!("\n{}", "The game was a draw!".bold().yellow());
            } else {
                match turn.win_data.0.unwrap() {
                    Player::X => {
                        println!("\n{}", "Player X won the game!".bold().red());
                    }
                    Player::O => {
                        println!("\n{}", "Player O won the game!".bold().blue());
                    }
                }
            }
            return Ok(());
        }
    }
}
