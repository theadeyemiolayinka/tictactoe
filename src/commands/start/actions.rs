use comfy_table::{Attribute, Cell, Table};

use crate::{
    AppResult, Failure, commands::start::game::Player, services::config::codes::ResultCode,
};

use super::game::{GameMatrix, GameMatrixWrapper, WinData};

pub const TIC_TAC_TOE_PRESET: &str = "     == |--        ";

pub const WIN: i32 = 10;
pub const LOSE: i32 = -10;
pub const DRAW: i32 = 0;

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_selectable_game_matrix(game_matrix: &GameMatrixWrapper) -> u16 {
    let gm = game_matrix.value().clone();
    let mut table = Table::new();
    table.load_preset(TIC_TAC_TOE_PRESET);

    let mut current_selectable: u16 = 0;

    for i in 0..gm.len() {
        let row: Vec<Cell> = gm[i]
            .iter()
            .map(|&cell| match cell {
                x if x == Player::X.as_i32() => Cell::new(" X ")
                    .fg(comfy_table::Color::Red)
                    .add_attribute(Attribute::Bold),
                x if x == Player::O.as_i32() => Cell::new(" O ")
                    .fg(comfy_table::Color::Blue)
                    .add_attribute(Attribute::Bold),
                _ => {
                    current_selectable += 1;
                    Cell::new(format!(" {} ", current_selectable))
                        .fg(comfy_table::Color::Yellow)
                        .add_attribute(Attribute::Bold)
                }
            })
            .collect();
        table.add_row(row);
    }

    table.set_width(30);

    println!("\n{}", table);
    current_selectable
}

pub fn print_final_game_matrix(game_matrix: &GameMatrixWrapper, win_cells: Vec<(usize, usize)>) {
    let gm = game_matrix.value().clone();
    let mut table = Table::new();
    table.load_preset(TIC_TAC_TOE_PRESET);

    for i in 0..gm.len() {
        let row: Vec<Cell> = gm[i]
            .iter()
            .enumerate()
            .map(|(idx, &cell)| match cell {
                x if x == Player::X.as_i32() => {
                    if win_cells.contains(&(i, idx)) {
                        Cell::new(" X ")
                            .fg(comfy_table::Color::White)
                            .bg(comfy_table::Color::Green)
                            .add_attribute(Attribute::Bold)
                    } else {
                        Cell::new(" X ")
                            .fg(comfy_table::Color::Red)
                            .add_attribute(Attribute::Bold)
                    }
                }
                x if x == Player::O.as_i32() => {
                    if win_cells.contains(&(i, idx)) {
                        Cell::new(" O ")
                            .fg(comfy_table::Color::White)
                            .bg(comfy_table::Color::Green)
                            .add_attribute(Attribute::Bold)
                    } else {
                        Cell::new(" O ")
                            .fg(comfy_table::Color::Blue)
                            .add_attribute(Attribute::Bold)
                    }
                }
                _ => {
                    if win_cells.contains(&(i, idx)) {
                        Cell::new("   ")
                            .fg(comfy_table::Color::White)
                            .bg(comfy_table::Color::Green)
                            .add_attribute(Attribute::Bold)
                    } else {
                        Cell::new("   ")
                            .fg(comfy_table::Color::Yellow)
                            .add_attribute(Attribute::Bold)
                    }
                }
            })
            .collect();
        table.add_row(row);
    }

    table.set_width(30);

    println!("\n{}", table);
}

pub fn select_position(
    game_matrix: &mut GameMatrixWrapper,
    position: u16,
    player: Player,
) -> AppResult<()> {
    let gm = game_matrix.value();
    let mut current_selectable: u16 = 0;

    for i in 0..gm.len() {
        for j in 0..gm[i].len() {
            match gm[i][j] {
                x if x == Player::X.as_i32() => {}
                x if x == Player::O.as_i32() => {}
                _ => {
                    current_selectable += 1;
                    if position == current_selectable {
                        game_matrix.set_position((i, j), player);
                        return Ok(());
                    }
                }
            }
        }
    }

    Err(Failure {
        message: "Invalid Position".to_string(),
        trace: "Select a valid position".to_string(),
        code: ResultCode::InvalidArgs,
    })
}

pub fn check_win(game_matrix: &mut GameMatrixWrapper) -> AppResult<WinData> {
    let gm = game_matrix.value();
    for i in 0..gm.len() {
        if gm[i][0] == gm[i][1] && gm[i][0] == gm[i][2] && gm[i][0] != 0 {
            return Ok((Player::from_i32(gm[i][0]), vec![(i, 0), (i, 1), (i, 2)]));
        }
        if gm[0][i] == gm[1][i] && gm[0][i] == gm[2][i] && gm[0][i] != 0 {
            return Ok((Player::from_i32(gm[0][i]), vec![(0, i), (1, i), (2, i)]));
        }
    }

    if gm[0][0] == gm[1][1] && gm[0][0] == gm[2][2] && gm[0][0] != 0 {
        return Ok((Player::from_i32(gm[0][0]), vec![(0, 0), (1, 1), (2, 2)]));
    }

    if gm[0][2] == gm[1][1] && gm[0][2] == gm[2][0] && gm[0][2] != 0 {
        return Ok((Player::from_i32(gm[0][2]), vec![(0, 2), (1, 1), (2, 0)]));
    }

    Ok((None, vec![]))
}

// AI ACTIONS

pub fn get_selectable(gm: &GameMatrix) -> u16 {
    let mut current_selectable: u16 = 0;

    for i in 0..gm.len() {
        for &x in gm[i].iter() {
            if x == Player::X.as_i32() {
            } else if x == Player::O.as_i32() {
            } else {
                current_selectable += 1;
            }
        }
    }
    current_selectable
}

pub fn ai_select(gm: &mut GameMatrix, pos: u16, player: Player) -> Option<(usize, usize)> {
    let mut current_selectable: u16 = 0;

    for i in 0..gm.len() {
        for (j, x) in gm[i].iter_mut().enumerate() {
            if *x == Player::X.as_i32() {
            } else if *x == Player::O.as_i32() {
            } else {
                current_selectable += 1;
                if pos == current_selectable {
                    *x = player.as_i32();
                    return Some((i, j));
                }
            }
        }
    }
    None
}

pub fn ai_unselect(gm: &mut GameMatrix, pos: (usize, usize)) {
    gm[pos.0][pos.1] = 0
}

pub fn check_win_slim(gm: &GameMatrix) -> Option<i32> {
    for i in 0..gm.len() {
        if gm[i][0] == gm[i][1] && gm[i][0] == gm[i][2] && gm[i][0] != 0 {
            return Some(gm[i][0]);
        }
        if gm[0][i] == gm[1][i] && gm[0][i] == gm[2][i] && gm[0][i] != 0 {
            return Some(gm[0][i]);
        }
    }

    if gm[0][0] == gm[1][1] && gm[0][0] == gm[2][2] && gm[0][0] != 0 {
        return Some(gm[0][0]);
    }

    if gm[0][2] == gm[1][1] && gm[0][2] == gm[2][0] && gm[0][2] != 0 {
        return Some(gm[0][2]);
    }

    None
}

pub fn evaluate_board(gm: &GameMatrix, player: Player) -> Option<i32> {
    if get_selectable(gm) > 0 {
        match check_win_slim(gm) {
            Some(g) => match g {
                x if x == player.as_i32() => return Some(WIN),
                _ => return Some(LOSE),
            },
            None => {
                return None;
            }
        }
    } else {
        match check_win_slim(gm) {
            Some(g) => match g {
                x if x == player.as_i32() => return Some(WIN),
                _ => return Some(LOSE),
            },
            None => {
                return Some(DRAW);
            }
        }
    }
}
