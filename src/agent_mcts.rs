use crate::agent_rand::agent as rand_agent;
use crate::connect4::{Action, Match, MatchResult, COLS};

pub fn agent(mat: &Match) -> Action {
    // For each action we could take, simulate multiple random games from the resulting state.
    // Keep track of the number of wins for each action.
    // Pick the action with the highest win rate.
    let mut max_score = f64::MIN;
    let mut best_action = Action { column: 0 };

    for col in 0..COLS {
        let action = Action { column: col };
        if mat.valid_action(&action) {
            let score = score_action(mat, &action);
            if score > max_score {
                max_score = score;
                best_action = action;
            }
        }
    }

    best_action
}

#[cfg(debug_assertions)]
const SIMULATIONS: i32 = 1_000;

#[cfg(not(debug_assertions))]
const SIMULATIONS: i32 = 10_000;

fn score_action(current_mat: &Match, action: &Action) -> f64 {
    let player = current_mat.next_player();

    // Create a new match with the action applied.
    let mut next_mat = current_mat.clone();
    next_mat.apply_action(action).unwrap();

    // Simulate random games from this state.
    let mut score = 0;
    for _ in 0..SIMULATIONS {
        let mut mat = next_mat.clone();
        match mat.play(rand_agent, rand_agent).unwrap() {
            MatchResult::Winner(winner) => {
                if winner == player {
                    score += 1;
                } else {
                    score -= 1;
                }
            }
            MatchResult::Tie => {}
        }
    }
    score as f64 / SIMULATIONS as f64
}
