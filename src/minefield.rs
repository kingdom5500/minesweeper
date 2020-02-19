use rand::seq::IteratorRandom;
use std::char;
use std::fmt;
use termion::color;

use crate::tile::{Tile, TileState};

const NUMBER_COLORS: [&dyn color::Color; 8] = [
    &color::LightBlue,
    &color::Green,
    &color::LightRed,
    &color::Blue,
    &color::Red,
    &color::Cyan,
    &color::White,
    &color::LightBlack,
];

#[derive(Debug, Eq, PartialEq)]
pub enum MineFieldState {
    Failed,
    Cleared,
    InProgress,
}

pub struct MineField {
    width: usize,
    height: usize,
    mines: usize,
    flags: usize,
    tiles: Vec<Tile>,
}

impl MineField {
    /// Create a new, empty minefield.
    pub fn empty(width: usize, height: usize) -> Self {
        let mut tiles = Vec::new();

        // Fill the vec with the correct amount of empty tiles.
        for _index in 0..(width * height) {
            tiles.push(Tile {
                state: TileState::Hidden,
                has_mine: false,
            });
        }

        Self {
            width: width,
            height: height,
            mines: 0,
            flags: 0,
            tiles: tiles,
        }
    }

    /// Populate the minefield with a given amount of mines.
    pub fn populate(&mut self, amount: usize) -> Result<(), &'static str> {
        // Get a vec of all the empty tiles that we can populate.
        let mut empty_tiles = Vec::new();

        for tile in self.tiles.iter_mut() {
            if !tile.has_mine {
                empty_tiles.push(tile)
            }
        }

        // Check if we have enough empty tiles to populate.
        if empty_tiles.len() < amount {
            return Err("Not enough space for those mines.");
        }

        // If we do, select some and populate them by index.
        let target_indices =
            rand::seq::index::sample(&mut rand::thread_rng(), empty_tiles.len(), amount);

        for index in target_indices.iter() {
            empty_tiles[index].has_mine = true;
        }

        self.mines += amount;

        Ok(())
    }

    /// Create a new minefield and populate it.
    pub fn new(width: usize, height: usize, mines: usize) -> Result<Self, &'static str> {
        let mut field = Self::empty(width, height);
        field.populate(mines)?;

        Ok(field)
    }

    /// Create a beginner field: 10x10 with 10 mines.
    #[inline]
    pub fn beginner() -> Self {
        Self::new(10, 10, 10).unwrap()
    }

    /// Create an intermediate field: 15x15 with 40 mines.
    #[inline]
    pub fn intermediate() -> Self {
        Self::new(15, 15, 40).unwrap()
    }

    /// Create an expert field: 30x16 with 99 mines.
    #[inline]
    pub fn expert() -> Self {
        Self::new(30, 16, 99).unwrap()
    }

    /// Access the tile width of the minefield.
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Access the tile height of the minefield.
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Access the amount of mines on the minefield.
    #[inline]
    pub fn mines(&self) -> usize {
        self.mines
    }

    #[inline]
    pub fn flags(&self) -> usize {
        self.flags
    }

    /// Check if a (row, column) is in range.
    #[inline]
    pub fn position_is_valid(&self, row: usize, column: usize) -> bool {
        row < self.height && column < self.width
    }

    /// Get an immutable reference to each tile in order.
    #[inline]
    pub fn iter_tiles(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }

    /// Get a mutable reference to each tile in order.
    #[inline]
    pub fn iter_mut_tiles(&mut self) -> impl Iterator<Item = &mut Tile> {
        self.tiles.iter_mut()
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let range = 0..(self.width * self.height);

        // map each index to the corresponding (row, column)
        range.map(move |index| (index / self.width, index % self.width))
    }

    /// Get an immutable reference to a specific tile.
    pub fn get_tile(&self, row: usize, column: usize) -> Result<&Tile, &'static str> {
        // check if the tile is in range, then fetch it.
        if self.position_is_valid(row, column) {
            Ok(&self.tiles[row * self.width + column])
        } else {
            Err("Tile not in range")
        }
    }

    /// Get a mutable reference to a specific tile.
    pub fn get_tile_mut(&mut self, row: usize, column: usize) -> Result<&mut Tile, &'static str> {
        if self.position_is_valid(row, column) {
            Ok(&mut self.tiles[row * self.width + column])
        } else {
            Err("Tile not in range")
        }
    }

    pub fn has_mine_at(&self, row: usize, column: usize) -> Result<bool, &'static str> {
        Ok(self.get_tile(row, column)?.has_mine)
    }

    pub fn get_tile_state(&self, row: usize, column: usize) -> Result<TileState, &'static str> {
        // TODO: go through the code and see where
        // this can be put to use.
        Ok(self.get_tile(row, column)?.state)
    }

    pub fn get_indices_near(
        &self,
        row: usize,
        column: usize,
    ) -> Result<Vec<(usize, usize)>, &'static str> {
        // Check if the tile even exists.
        if let Err(e) = self.get_tile(row, column) {
            return Err(e);
        }

        // This is a silly case, but whatever:
        if self.width == 1 && self.height == 1 {
            return Ok(Vec::new());
        }

        // Now get the indices of adjacent rows.
        let (min_row, max_row) = if row == 0 {
            (row, row + 1) // top edge
        } else if row == self.height - 1 {
            (row - 1, row) // bottom edge
        } else {
            (row - 1, row + 1) // in between
        };

        // ... and the same with adjacent columns
        let (min_column, max_column) = if column == 0 {
            (column, column + 1) // left edge
        } else if column == self.width - 1 {
            (column - 1, column) // right edge
        } else {
            (column - 1, column + 1) // in between
        };

        // then create a vec of all the indices
        let mut indices = Vec::new();

        for adj_row in min_row..(max_row + 1) {
            for adj_column in min_column..(max_column + 1) {
                if adj_row == row && adj_column == column {
                    continue; // Skip the tile we're actually looking around
                }

                indices.push((adj_row, adj_column));
            }
        }

        Ok(indices)
    }

    pub fn get_tiles_near(&self, row: usize, column: usize) -> Result<Vec<&Tile>, &'static str> {
        // convert indices to their corresponding tiles
        self.get_indices_near(row, column)?
            .iter()
            .map(|(r, c)| self.get_tile(*r, *c))
            .collect()
    }

    /// Count the amount of mines near a tile.
    pub fn count_mines_near(&self, row: usize, column: usize) -> Result<usize, &'static str> {
        Ok(self
            .get_tiles_near(row, column)?
            .iter()
            .filter(|tile| tile.has_mine)
            .count())
    }

    pub fn has_mines_near(&self, row: usize, column: usize) -> Result<bool, &'static str> {
        Ok(self
            .get_tiles_near(row, column)?
            .iter()
            .any(|&tile| tile.has_mine))
    }

    /// Toggle a tile state between `Hidden` and `Flagged`.
    pub fn toggle_flag(&mut self, row: usize, column: usize) -> Result<(), &'static str> {
        let mut tile = self.get_tile_mut(row, column)?;

        match tile.state {
            TileState::Hidden => {
                tile.state = TileState::Flagged;
                self.flags += 1;
            }
            TileState::Flagged => {
                tile.state = TileState::Hidden;
                self.flags -= 1;
            }
            _ => (),
        }

        Ok(())
    }

    /// Change a tile state from `Hidden` to `Visible`.
    pub fn dig_tile(&mut self, row: usize, column: usize) -> Result<(), &'static str> {
        let mut tile = self.get_tile_mut(row, column)?;

        match tile.state {
            TileState::Hidden => tile.state = TileState::Visible,
            _ => (),
        }

        Ok(())
    }

    /// Get the char representation of a tile.
    pub fn char_for_tile(&self, row: usize, column: usize) -> Result<String, &'static str> {
        let tile = self.get_tile(row, column)?;

        // TODO: might be nice to make these customisable at some point.
        Ok(match tile.state {
            TileState::Hidden => String::from("#"),
            TileState::Flagged => format!(
                "{}~{}",
                color::Fg(color::LightMagenta),
                color::Fg(color::Reset),
            ),
            TileState::Visible if tile.has_mine => String::from("X"),
            TileState::Visible => {
                match self.count_mines_near(row, column).unwrap() {
                    // if the tile is exposed and empty, show the empty
                    // tile or display the amount of surrounding mines
                    0 => String::from(" "),
                    n => format!(
                        "{}{}{}",
                        color::Fg(NUMBER_COLORS[n - 1]),
                        char::from_digit(n as u32, 10).unwrap(),
                        color::Fg(color::Reset),
                    ),
                }
            }
        })
    }

    /// Perform a flood fill on empty space.
    pub fn flood_empty_tiles(&mut self, row: usize, column: usize) -> Result<(), &'static str> {
        if self.get_tile_state(row, column)? == TileState::Visible {
            return Ok(());
        }

        self.dig_tile(row, column)?;

        if !self.has_mines_near(row, column)? {
            let indices = self.get_indices_near(row, column)?;

            for (adj_row, adj_column) in indices.iter() {
                self.flood_empty_tiles(*adj_row, *adj_column)?
            }
        }

        Ok(())
    }

    /// Open a random empty field for convenience,
    /// then return the index of a tile within it.
    pub fn clear_first_opening(&mut self) -> Option<(usize, usize)> {
        let mut target_indices = Vec::new();

        // search for potentially empty fields
        for (row, column) in self.iter_positions() {
            let near_mines = self.has_mines_near(row, column).unwrap();
            let is_mine = self.has_mine_at(row, column).unwrap();

            // if this tile is far from mines, keep track of it.
            if !is_mine && !near_mines {
                target_indices.push((row, column))
            }
        }

        // select a random empty tile to open.
        let mut rng = rand::thread_rng();
        let target_tile = target_indices.iter().choose(&mut rng);

        if let Some((row, column)) = target_tile {
            self.flood_empty_tiles(*row, *column).unwrap();
            return Some((*row, *column));
        }

        None
    }

    /// Perform what's known as a "chording" move.
    ///
    /// This is where a tile is surrounded by the same
    /// amount of flags as mines. All other tiles are
    /// then assumed to be safe, and are uncovered.
    pub fn do_chord(&mut self, row: usize, column: usize) -> Result<(), &'static str> {
        let this_tile = self.get_tile(row, column)?;
        if this_tile.state != TileState::Visible {
            return Ok(());
        }

        let mut nearby_flags = 0;
        let mut nearby_mines = 0;

        let mut hidden_indices = Vec::new();

        // count up the number of flags and mines near the tile.
        for (adj_row, adj_column) in self.get_indices_near(row, column)? {
            let tile = self.get_tile(adj_row, adj_column)?;

            match tile.state {
                TileState::Flagged => nearby_flags += 1,
                TileState::Hidden => hidden_indices.push((adj_row, adj_column)),
                _ => (),
            }

            if tile.has_mine {
                nearby_mines += 1;
            }
        }

        // if they are equal, clear everything else around the tile.
        if nearby_flags == nearby_mines {
            for (adj_row, adj_column) in hidden_indices.iter_mut() {
                self.flood_empty_tiles(*adj_row, *adj_column)?;
            }
        }

        Ok(())
    }

    /// Work out the state of the game.
    pub fn get_state(&self) -> MineFieldState {
        let mut is_cleared = true;

        for tile in self.iter_tiles() {
            match tile.state {
                // if a mine is exposed, they've failed
                TileState::Visible if tile.has_mine => return MineFieldState::Failed,

                // or if a tile is still unsolved, they haven't cleared.
                TileState::Hidden if !tile.has_mine => is_cleared = false,

                _ => (),
            }
        }

        if is_cleared {
            MineFieldState::Cleared
        } else {
            MineFieldState::InProgress
        }
    }

    /// Make all tiles visible except correct flags.
    pub fn game_over(&mut self) {
        for tile in self.iter_mut_tiles() {
            let bad_flag = tile.state == TileState::Flagged && tile.has_mine;

            if bad_flag || tile.state == TileState::Hidden {
                tile.state = TileState::Visible;
            }
        }
    }
}

/// Allow the minefield to be printed to the console.
impl fmt::Display for MineField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut chars = Vec::new();
        let mut output = String::new();

        for (row, column) in self.iter_positions() {
            chars.push(self.char_for_tile(row, column).unwrap());
        }

        for (index, string) in chars.iter().enumerate() {
            // separate rows with newline chars.
            if index != 0 && index % self.width == 0 {
                output.push_str("\r\n");
            }

            output.push_str(string);
            output.push(' ');
        }

        write!(f, "{}", output)
    }
}
