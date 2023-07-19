use std::io;

use crate::connect4::{Action, MatchState, Player};

mod agent_mcts;
mod agent_rand;
mod connect4;
mod tui;

enum Agent {
    Human,
    Random,
    Mcts,
}

fn play(blue: Agent, red: Agent) -> io::Result<()> {
    let mut mat = connect4::Match::default();
    let mut state = mat.state();
    while matches!(state, MatchState::InProgress) {
        let agent = match mat.next_player {
            Player::Blue => &blue,
            Player::Red => &red,
        };
        let action = match agent {
            Agent::Human => {
                tui::show_match(&mat, true)?;
                'turn: loop {
                    let c = tui::read_char()?;
                    if c == 'q' {
                        return Ok(());
                    }
                    if let Some(c) = c.to_digit(10) {
                        if c == 0 || c > 7 {
                            continue 'turn;
                        }
                        let action = Action {
                            column: (c - 1) as usize,
                        };
                        if mat.valid_action(&action) {
                            break 'turn action;
                        }
                    }
                }
            }
            Agent::Random => {
                tui::show_match(&mat, false)?;
                agent_rand::agent(&mat)
            }
            Agent::Mcts => {
                tui::show_match(&mat, false)?;
                agent_mcts::agent(&mat)
            }
        };
        state = mat.apply_action(&action).unwrap();
    }
    tui::show_match(&mat, false)?;
    while tui::read_char()? != 'q' {}
    Ok(())
}

fn main() -> io::Result<()> {
    tui::setup()?;
    loop {
        tui::main_menu()?;
        match tui::read_char()? {
            '1' => play(Agent::Human, Agent::Human)?,
            '2' => play(Agent::Human, Agent::Random)?,
            '3' => play(Agent::Human, Agent::Mcts)?,
            'q' => {
                break;
            }
            _ => (),
        }
    }
    tui::cleanup()
}
