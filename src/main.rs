extern crate rustbox;

mod game;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
