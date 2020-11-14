use rustbox::{Key, RustBox};
use rand::prelude::*;
use rand_pcg::Pcg64;

type RNG = Pcg64;

pub mod render;
pub mod animals;
pub mod entity;
pub mod position;
pub mod player;
pub mod map;

use render::render;
use entity::*;
use player::*;
use animals::*;
use map::*;
use position::*;

pub struct Game {
    player: Player,
    entities: Vec<Animal>,
    rustbox: RustBox,
    map: GameMap,
    rng: RNG,
    tick_count: usize
}

impl Game {
    pub fn new() -> Game {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("Failed: {}", e),
        };
        let map = GameMap::new();
        let map_center = GameMap::center() as i32;
        let mut game = Game {
            player: Player::new(map_center, map_center),
            rustbox,
            rng: Pcg64::seed_from_u64(1234),
            map,
            entities: Vec::new(),
            tick_count: 0
        };
        game.map.generate(&mut game.rng);
        for _ in 0..10 {
            let animal = Animal::new(map_center + game.rng.gen_range(-50, 50), map_center + game.rng.gen_range(-50, 50));
            game.entities.push(animal);
        }
        game
    }

    pub fn player_position(&self) -> &Position {
        self.player.current_position()
    }

    pub fn process_input(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('q') => {
                return false;
            }
            Key::Left => {
                self.player.position().move_relative(-1, 0)
            }
            Key::Right => {
                self.player.position().move_relative(1, 0)
            }
            Key::Up => {
                self.player.position().move_relative(0, -1)
            }
            Key::Down => {
                self.player.position().move_relative(0, 1)
            }
            _ => {}
        }
        return true;
    }

    pub fn run(&mut self) {
        loop {
            render(&self.rustbox, &self);
            match self.rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => {
                    let continue_game = self.process_input(&key);
                    if !continue_game { break; }
                },
                Err(e) => panic!("{}", e.to_string()),
                _ => {}
            }
            let mut player = self.player;
            player.tick(self);
            self.player = player;
            for i in 0..self.entities.len() {
                let mut entity = self.entities[i];
                entity.tick(self);
                self.entities[i] = entity;
            }
            self.tick_count += 1;
        }
    }
}
