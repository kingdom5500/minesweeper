#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TileState {
    Hidden,
    Visible,
    Flagged,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Tile {
    pub state: TileState,
    pub has_mine: bool,
}
