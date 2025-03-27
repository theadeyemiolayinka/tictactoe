use std::collections::HashMap;

use crate::commands::start::actions::WIN;

use super::{
    actions::{ai_select, ai_unselect, evaluate_board, get_selectable, select_position},
    game::{GameMatrix, GameMatrixWrapper, Player},
};

pub fn make_move(gm: &mut GameMatrixWrapper, player: Player) {
    let possibilities = get_selectable(&gm.value());

    let mut check_matrix = gm.value();

    // Check for an immediate win
    for i in 1..=possibilities {
        if let Some(pos) = ai_select(&mut check_matrix, i, player) {
            if evaluate_board(&check_matrix, player) == Some(WIN) {
                ai_unselect(&mut check_matrix, pos); // Undo move
                select_position(gm, i, player).unwrap();
                return;
            }
            ai_unselect(&mut check_matrix, pos);
        }
    }

    // Check for an immediate block (opponent's win)
    let opponent = player.invert();
    for i in 1..=possibilities {
        if let Some(pos) = ai_select(&mut check_matrix, i, opponent) {
            if evaluate_board(&check_matrix, opponent) == Some(WIN) {
                ai_unselect(&mut check_matrix, pos); // Undo move
                select_position(gm, i, player).unwrap();
                return;
            }
            ai_unselect(&mut check_matrix, pos);
        }
    }

    // Use Minimax for Best Move
    let mut best_scores: HashMap<u16, i32> = (1..=possibilities).map(|g| (g, 0)).collect();
    generate_min_max_choices(&mut gm.value(), player, player, &mut best_scores, 0);

    let mut action_map: Vec<(u16, i32)> = best_scores.iter().map(|g| (*g.0, *g.1)).collect();
    action_map.sort_by(|a, b| b.1.cmp(&a.1));

    let selected_move = action_map[0].0;
    select_position(gm, selected_move, player).unwrap();
}

fn generate_min_max_choices(
    gm: &mut GameMatrix,
    player: Player,
    ai_player: Player,
    best_scores: &mut HashMap<u16, i32>,
    depth: i32,
) -> i32 {
    match evaluate_board(gm, player) {
        Some(score) => {
            return score - depth;
        }
        None => {}
    };

    let spaces: u16 = get_selectable(&gm);
    let mut best_score = if player == ai_player {
        i32::MIN
    } else {
        i32::MAX
    };

    for i in 1..=spaces {
        if let Some(pos) = ai_select(gm, i, player) {
            let score =
                generate_min_max_choices(gm, player.invert(), ai_player, best_scores, depth + 1);

            if depth == 0 {
                best_scores.insert(i, score);
            }

            if player == ai_player {
                best_score = best_score.max(score);
            } else {
                best_score = best_score.min(score);
            }

            ai_unselect(gm, pos);
        }
    }

    best_score
}
