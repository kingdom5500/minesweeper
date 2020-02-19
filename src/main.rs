use std::env;

mod game;
mod minefield;
mod tile;

use game::Minesweeper;

fn custom_game(config: String) -> Minesweeper {
    let first_split: Vec<&str> = config.split('_').collect();

    if first_split.len() != 2 {
        panic!("Expected format: 'WxH_M'.");
    }

    let geom = first_split[0];
    let mines: usize = first_split[1]
        .parse()
        .expect("Mines must be a positive integer.");

    let geom_split: Vec<&str> = geom.split('x').collect();

    if geom_split.len() != 2 {
        panic!("Expected format: 'WxH_M'.");
    }

    let width: usize = geom_split[0]
        .parse()
        .expect("Width must be a positive integer.");

    let height: usize = geom_split[1]
        .parse()
        .expect("Height must be a positive integer.");

    Minesweeper::new(width, height, mines).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut minesweeper = if args.len() <= 1 {
        Minesweeper::beginner()
    } else {
        match args[1].trim() {
            "beginner" => Minesweeper::beginner(),
            "intermediate" => Minesweeper::intermediate(),
            "expert" => Minesweeper::expert(),
            "custom" => custom_game(args[2].clone()),
            _ => panic!("Unknown game difficulty."),
        }
    };

    minesweeper.play();
}
