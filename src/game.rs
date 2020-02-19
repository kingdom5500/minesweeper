use std::io::{stdin, stdout, Stdout, Write};
use std::time::{Duration, SystemTime};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::minefield::{MineField, MineFieldState};

pub struct Minesweeper {
    field: MineField,
}

/// Wait for a specific key to be pressed.
fn wait_for_key(target_key: Key) {
    for key in stdin().keys() {
        if key.unwrap() == target_key {
            break;
        }
    }
}

/// Write some text at a specific position on the console.
fn write_text(raw_stdout: &mut RawTerminal<Stdout>, string: String, x_pos: u16, y_pos: u16) {
    print!(
        "{}{}{}{}",
        termion::cursor::Save,
        termion::cursor::Goto(x_pos, y_pos),
        string,
        termion::cursor::Restore,
    );

    raw_stdout.flush().unwrap();
}

/// Methods for the text-based interface of the game.
impl Minesweeper {
    /// Set up a game with a pre-defined field.
    pub fn with_field(field: MineField) -> Self {
        Self { field: field }
    }

    /// Set up a fully new, random game.
    pub fn new(rows: usize, columns: usize, mines: usize) -> Result<Self, &'static str> {
        let field = MineField::new(rows, columns, mines)?;
        Ok(Self::with_field(field))
    }

    #[inline]
    pub fn beginner() -> Self {
        Self::with_field(MineField::beginner())
    }

    #[inline]
    pub fn intermediate() -> Self {
        Self::with_field(MineField::intermediate())
    }

    #[inline]
    pub fn expert() -> Self {
        Self::with_field(MineField::expert())
    }

    /// Write text centred below the field.
    fn write_text_below(
        &self,
        raw_stdout: &mut RawTerminal<Stdout>,
        string: String,
        lines_below: u16,
    ) {
        let x_offset = self.field.width().saturating_sub(string.len() / 2) as u16;

        let y_offset = self.field.height() as u16 + lines_below + 1;

        write_text(raw_stdout, string, x_offset, y_offset)
    }

    /// Write text to the right of the field.
    fn write_text_beside(&self, raw_stdout: &mut RawTerminal<Stdout>, string: String, line: u16) {
        write_text(
            raw_stdout,
            string,
            self.field.width() as u16 * 2 + 2,
            line + 1,
        );
    }

    /// Display the standard text beside the field.
    fn display_side_text(&self, mut raw_stdout: &mut RawTerminal<Stdout>) {
        let game_text = format!(
            "{}x{} field with {} mines",
            self.field.width(),
            self.field.height(),
            self.field.mines()
        );

        let flags_text = format!("{} flags used", self.field.flags());

        self.write_text_beside(&mut raw_stdout, game_text, 0);
        self.write_text_beside(&mut raw_stdout, flags_text, 1);
    }

    /// Clear the console and display the field.
    fn redraw_field(
        &self,
        mut raw_stdout: &mut RawTerminal<Stdout>,
        tile_row: u16,
        tile_column: u16,
    ) {
        // first clear the screen and redraw the field
        print!(
            "{}{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
            self.field,
            termion::cursor::Goto(tile_column * 2 + 1, tile_row + 1),
        );

        self.display_side_text(&mut raw_stdout);
    }

    /// Pause the game and keep track of the pause duration.
    fn pause_game(&self, mut raw_stdout: &mut RawTerminal<Stdout>) -> Duration {
        self.write_text_below(
            &mut raw_stdout,
            String::from("Paused! Press 'p' to unpause."),
            1,
        );

        let paused = SystemTime::now();
        wait_for_key(Key::Char('p'));
        let unpaused = SystemTime::now();

        return unpaused.duration_since(paused).unwrap();
    }

    /// Play a full round of the game with the interface.
    pub fn play(&mut self) {
        // it would be ideal to have this be more detached from the
        // user interface to some degree, but it should be fine.

        // set up the first open field before displaying.
        let (start_row, start_column) = self.field.clear_first_opening().unwrap_or((0, 0));

        let mut tile_row = start_row as u16;
        let mut tile_column = start_column as u16;
        let mut check_for_mine = false;

        let start_time = SystemTime::now();
        let mut paused_time = Duration::new(0, 0);

        let mut raw_stdout = stdout().into_raw_mode().unwrap();
        self.redraw_field(&mut raw_stdout, tile_row, tile_column);
        self.display_side_text(&mut raw_stdout);

        // this will loop instantly when a key is pressed.
        for key in stdin().keys() {
            match key.unwrap() {
                // cursor controls
                Key::Up => tile_row = tile_row.saturating_sub(1),
                Key::Down => tile_row = tile_row.saturating_add(1),
                Key::Left => tile_column = tile_column.saturating_sub(1),
                Key::Right => tile_column = tile_column.saturating_add(1),

                // tile controls. toggles a flag.
                Key::Char('f') => self
                    .field
                    .toggle_flag(tile_row as usize, tile_column as usize)
                    .unwrap(),

                // digs an empty space.
                Key::Char(' ') => {
                    self.field
                        .flood_empty_tiles(tile_row as usize, tile_column as usize)
                        .unwrap();

                    check_for_mine = true
                }

                // performs a chording move.
                Key::Char('d') => {
                    self.field
                        .do_chord(tile_row as usize, tile_column as usize)
                        .unwrap();

                    check_for_mine = true
                }

                // miscellaneous controls
                Key::Char('p') => paused_time += self.pause_game(&mut raw_stdout),
                Key::Char('q') => break,
                _ => continue,
            };

            // ensure that the cursor stays in range.
            if tile_row >= self.field.height() as u16 {
                tile_row = self.field.height() as u16 - 1
            }

            if tile_column >= self.field.width() as u16 {
                tile_column = self.field.width() as u16 - 1
            }

            // if a space has been cleared, there may be a mine.
            if check_for_mine {
                // check if the game has been finished.
                if self.field.get_state() != MineFieldState::InProgress {
                    break;
                };

                check_for_mine = false;
            }

            // redraw the field after every key event.
            self.redraw_field(&mut raw_stdout, tile_row, tile_column);
            self.display_side_text(&mut raw_stdout);
        }

        self.field.game_over();
        self.redraw_field(&mut raw_stdout, tile_row, tile_column);
        self.display_side_text(&mut raw_stdout);

        let time_taken = SystemTime::now().duration_since(start_time).unwrap() - paused_time;
        let time_text = format!("You took {} seconds", time_taken.as_secs());

        // hide the cursor and wait for a keypress to finish.
        print!("{}", termion::cursor::Hide);

        self.write_text_below(&mut raw_stdout, time_text, 1);
        self.write_text_below(&mut raw_stdout, String::from("Press 'q' to finish"), 2);
        wait_for_key(Key::Char('q'));

        // clear the screen upon completion.
        print!(
            "{}{}{}",
            termion::cursor::Show,
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        );

        raw_stdout.flush().unwrap();
    }
}
