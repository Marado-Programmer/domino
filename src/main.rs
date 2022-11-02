use crate::game::Game;
use std::error::Error;

pub mod game;

fn main() -> Result<(), Box<dyn Error>> {
    Game::start(2, Some(1));

    Ok(())
}
