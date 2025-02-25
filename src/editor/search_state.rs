use editor::cursor::Cursor;
use editor::row::Highlight;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

pub struct SearchState {
    pub last_match: Option<Match>,
    pub direction: Direction,
}

pub struct Match {
    pub cursor: Cursor,
    pub highlight: Vec<Highlight>,
}

impl SearchState {
    pub fn new() -> Self {
        SearchState {
            last_match: None,
            direction: Direction::Forward,
        }
    }
}
