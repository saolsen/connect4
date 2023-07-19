use crate::connect4::Player::Blue;
use thiserror::Error;

pub const ROWS: usize = 6;
pub const COLS: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Blue,
    Red,
}

impl Player {
    pub fn next(&self) -> Player {
        use Player::*;
        match self {
            Blue => Red,
            Red => Blue,
        }
    }
}

#[derive(Error, Debug)]
pub enum ActionError {
    #[error("Column must be between 0 and 6. Got `{0}`.")]
    UnknownColumn(usize),
    #[error("Column `{0}` is full.")]
    FullColumn(usize),
}

#[derive(Debug)]
pub struct Action {
    pub column: usize,
}

#[derive(Debug)]
pub enum MatchResult {
    Winner(Player),
    Tie,
}

#[derive(Debug)]
pub enum MatchState {
    InProgress,
    Over(MatchResult),
}

#[derive(Debug, Clone)]
pub struct Match {
    board: Vec<Option<Player>>,
    next_player: Player,
}

impl Default for Match {
    fn default() -> Self {
        Self {
            board: vec![None; ROWS * COLS],
            next_player: Blue,
        }
    }
}

impl Match {
    pub fn next_player(&self) -> Player {
        self.next_player
    }

    pub fn get(&self, col: usize, row: usize) -> Option<Player> {
        self.board[col * ROWS + row]
    }

    pub fn set(&mut self, col: usize, row: usize, val: Option<Player>) {
        self.board[col * ROWS + row] = val;
    }

    #[allow(clippy::identity_op)]
    pub fn state(&self) -> MatchState {
        use MatchResult::*;
        use MatchState::*;
        // Check vertical wins
        for col in 0..COLS {
            for row in 0..3 {
                match (
                    self.get(col, row + 0),
                    self.get(col, row + 1),
                    self.get(col, row + 2),
                    self.get(col, row + 3),
                ) {
                    (Some(i), Some(j), Some(k), Some(l)) if i == j && j == k && k == l => {
                        return Over(Winner(i))
                    }
                    _ => (),
                }
            }
        }

        // Check horizontal wins
        for row in 0..ROWS {
            for col in 0..4 {
                match (
                    self.get(col + 0, row),
                    self.get(col + 1, row),
                    self.get(col + 2, row),
                    self.get(col + 3, row),
                ) {
                    (Some(i), Some(j), Some(k), Some(l)) if i == j && j == k && k == l => {
                        return Over(Winner(i))
                    }
                    _ => (),
                }
            }
        }

        // Check diagonal up wins
        for col in 0..4 {
            for row in 0..3 {
                match (
                    self.get(col + 0, row + 0),
                    self.get(col + 1, row + 1),
                    self.get(col + 2, row + 2),
                    self.get(col + 3, row + 3),
                ) {
                    (Some(i), Some(j), Some(k), Some(l)) if i == j && j == k && k == l => {
                        return Over(Winner(i))
                    }
                    _ => (),
                }
            }
        }

        // Check diagonal down wins
        for col in 0..4 {
            for row in 3..6 {
                match (
                    self.get(col + 0, row - 0),
                    self.get(col + 1, row - 1),
                    self.get(col + 2, row - 2),
                    self.get(col + 3, row - 3),
                ) {
                    (Some(i), Some(j), Some(k), Some(l)) if i == j && j == k && k == l => {
                        return Over(Winner(i))
                    }
                    _ => (),
                }
            }
        }

        // Check for tie
        for col in 0..COLS {
            if self.get(col, ROWS - 1).is_none() {
                return InProgress;
            }
        }

        Over(Tie)
    }

    pub fn valid_action(&self, action: &Action) -> bool {
        if action.column >= COLS {
            return false;
        }
        self.get(action.column, ROWS - 1).is_none()
    }

    pub fn apply_action(&mut self, action: &Action) -> Result<MatchState, ActionError> {
        use ActionError::*;
        if action.column >= COLS {
            return Err(UnknownColumn(action.column));
        }
        for row in 0..ROWS {
            if self.get(action.column, row).is_none() {
                self.set(action.column, row, Some(self.next_player));
                self.next_player = self.next_player.next();
                return Ok(self.state());
            }
        }
        Err(FullColumn(action.column))
    }

    pub fn play(
        &mut self,
        blue_player: fn(&Match) -> Action,
        red_player: fn(&Match) -> Action,
    ) -> Result<MatchResult, ActionError> {
        loop {
            use Player::*;
            let action = match &self.next_player {
                Blue => blue_player(self),
                Red => red_player(self),
            };
            let state = self.apply_action(&action)?;
            if let MatchState::Over(result) = state {
                return Ok(result);
            }
        }
    }
}
