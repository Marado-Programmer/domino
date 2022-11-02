use crate::game::Game;

pub mod game;

fn main() {
    Game::start(2, Some(1));
}
