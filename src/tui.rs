use std::io;
use std::io::Write;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crossterm::{
    cursor,
    event::{self, Event},
    execute, queue, style,
    terminal::{self, ClearType},
};

use crate::connect4::{Action, Match, MatchResult, MatchState, Player, COLS, ROWS};

pub fn read_char() -> io::Result<char> {
    loop {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        }) = event::read()?
        {
            return Ok(c);
        }
    }
}

const BORDER: &str = "+---+---+---+---+---+---+---+";

pub fn show_match(mat: &Match, your_turn: bool) -> io::Result<()> {
    let mut stdout = io::stdout();

    // Header
    queue!(
        stdout,
        terminal::Clear(ClearType::All),
        style::SetForegroundColor(style::Color::Black),
        style::SetBackgroundColor(style::Color::White),
        cursor::MoveTo(0, 0)
    )?;

    let state = mat.state();
    match state {
        MatchState::InProgress => {
            match mat.next_player {
                Player::Blue => {
                    queue!(
                        stdout,
                        style::SetForegroundColor(style::Color::Blue),
                        style::Print("Blue"),
                    )?;
                }
                Player::Red => {
                    queue!(
                        stdout,
                        style::SetForegroundColor(style::Color::Red),
                        style::Print("Red"),
                    )?;
                }
            }
            queue!(stdout, style::ResetColor, style::Print("'s turn"),)?;
            if your_turn {
                queue!(stdout, style::Print(" (that's you)"))?;
            }
            queue!(stdout, cursor::MoveToNextLine(1))?;
        }
        MatchState::Over(result) => match result {
            MatchResult::Winner(winner) => {
                match winner {
                    Player::Blue => {
                        queue!(
                            stdout,
                            style::SetForegroundColor(style::Color::Blue),
                            style::Print("Blue"),
                        )?;
                    }
                    Player::Red => {
                        queue!(
                            stdout,
                            style::SetForegroundColor(style::Color::Red),
                            style::Print("Red"),
                        )?;
                    }
                }
                queue!(
                    stdout,
                    style::ResetColor,
                    style::Print(" wins!"),
                    cursor::MoveToNextLine(1)
                )?;
            }
            MatchResult::Tie => {
                queue!(
                    stdout,
                    style::Print("It's a Tie"),
                    cursor::MoveToNextLine(1)
                )?;
            }
        },
    }

    if your_turn {
        for column in 0..COLS {
            if mat.valid_action(&Action { column }) {
                queue!(stdout, style::Print(format!("  {} ", column + 1)))?;
            } else {
                queue!(stdout, style::Print("    "))?;
            }
        }
    }
    queue!(
        stdout,
        cursor::MoveToNextLine(1),
        style::Print(BORDER),
        cursor::MoveToNextLine(1)
    )?;
    for row in (0..ROWS).rev() {
        for column in 0..COLS {
            queue!(stdout, style::Print("| "))?;
            match mat.board[column * ROWS + row] {
                Some(Player::Blue) => {
                    queue!(
                        stdout,
                        style::SetForegroundColor(style::Color::Blue),
                        style::Print("●"),
                        style::ResetColor
                    )?;
                }
                Some(Player::Red) => {
                    queue!(
                        stdout,
                        style::SetForegroundColor(style::Color::Red),
                        style::Print("●"),
                        style::ResetColor
                    )?;
                }
                None => {
                    queue!(stdout, style::Print(" "))?;
                }
            };
            queue!(stdout, style::Print(" "))?;
        }
        queue!(
            stdout,
            style::Print("|"),
            cursor::MoveToNextLine(1),
            style::Print(BORDER),
            cursor::MoveToNextLine(1)
        )?;
    }

    if your_turn {
        queue!(stdout, style::Print("choose a column (1-7) or "),)?;
    }

    queue!(
        stdout,
        style::Print("hit 'q' to quit"),
        cursor::MoveToNextLine(1),
        style::ResetColor
    )?;

    stdout.flush()
}

const MENU: &str = r#"Choose an opponent.

1. Human. (Yourself or the person next to you).
2. Random agent.
3. Monte Carlo Tree Search agent.

Select opponent ('1', '2', or '3') or hit 'q' to quit.
"#;

pub fn main_menu() -> io::Result<()> {
    let mut stdout = io::stdout();
    queue!(
        stdout,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0),
        style::Print("Connect 4")
    )?;

    #[cfg(debug_assertions)]
    queue!(
        stdout,
        style::SetForegroundColor(style::Color::Red),
        style::Print(" !!! Debug Mode (mcts agent is weaker) !!!"),
        style::ResetColor
    )?;

    queue!(stdout, cursor::MoveToNextLine(1))?;

    for line in MENU.split('\n') {
        queue!(stdout, style::Print(line), cursor::MoveToNextLine(1))?;
    }

    stdout.flush()
}

pub fn setup() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()
}

pub fn cleanup() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()
}
