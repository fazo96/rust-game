use rustbox::{Key, RustBox};

pub mod render;

pub struct Entity {
    pub x: i32,
    pub y: i32,
}

pub struct Game {
    pub player: Entity,
    rustbox: RustBox,
    map: [i32; 10000]
}

impl Game {
    pub fn new() -> Game {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("Failed: {}", e),
        };
        let player = Entity { x: 100, y: 100 };
        Game {
            player: player,
            rustbox: rustbox,
            map: [0; 10000]
        }
    }

    pub fn process_input(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('q') => {
                return false;
            }
            Key::Left => {
                self.player.x -= 1;
            }
            Key::Right => {
                self.player.x += 1;
            }
            Key::Up => {
                self.player.y -= 1;
            }
            Key::Down => {
                self.player.y += 1;
            }
            _ => {}
        }
        return true;
    }

    pub fn run(&mut self) {
        loop {
            render::render(&self.rustbox, &self);
            match self.rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => {
                    let continue_game = self.process_input(&key);
                    if !continue_game { break; }
                },
                Err(e) => panic!("{}", e.to_string()),
                _ => {}
            }
        }
    }
}
