extern crate rustbox;

use std::default::Default;

use rustbox::RustBox;

mod game;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let player = game::Entity { x: 0, y: 0 };
    let mut game = game::Game { player: player };
    loop {
        game::render::render(&rustbox, &game);
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                let continue_game = game::process_input(&key, &mut game);
                if !continue_game { break; }
            },
            Err(e) => panic!("{}", e.to_string()),
            _ => {}
        }
    }
}
