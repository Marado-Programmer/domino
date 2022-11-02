use std::error::Error;
use crate::game::Game;

pub mod game;

fn main() -> Result<(), Box<dyn Error>> {
    Game::start(2, Some(1));

    Ok(())
}
