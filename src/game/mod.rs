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

use std::fmt;
use render::render;
use entity::*;
use player::*;
use animals::*;
use map::*;
use position::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GameState {
    Gameplay,
    Quit,
    InspectTiles
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct Game {
    state: GameState,
    player: Player,
    entities: Vec<Animal>,
    rustbox: RustBox,
    map: GameMap,
    rng: RNG,
    tick_count: usize,
    last_input_key: Option<Key>
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
            state: GameState::Gameplay,
            player: Player::new(map_center, map_center),
            rustbox,
            rng: Pcg64::seed_from_u64(1234),
            map,
            entities: Vec::new(),
            tick_count: 0,
            last_input_key: None
        };
        game.map.generate(&mut game.rng);
        for _ in 0..10 {
            let animal = Animal::new(map_center + game.rng.gen_range(-50, 50), map_center + game.rng.gen_range(-50, 50));
            game.entities.push(animal);
        }
        game
    }

    pub fn current_state(&self) -> GameState {
        self.state
    }

    pub fn player_position(&self) -> &Position {
        self.player.current_position()
    }

    pub fn camera_position(&self) -> &Position {
        match self.state {
            GameState::InspectTiles => self.player.cursor_position(),
            _ => self.player.current_position()
        }
    }

    pub fn process_input(&mut self, key: &Key) {
        self.last_input_key = Some(key.clone());
    }

    pub fn tick_count(&self) -> usize {
        self.tick_count
    }

    pub fn run(&mut self) {
        while self.state != GameState::Quit {
            render(&self.rustbox, &self);
            match self.rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => {
                    self.process_input(&key);
                },
                Err(e) => panic!("{}", e.to_string()),
                _ => {}
            }
            let mut player = self.player;
            player.tick(self);
            self.player = player;
            if self.state == GameState::Gameplay {
                for i in 0..self.entities.len() {
                    let mut entity = self.entities[i];
                    entity.tick(self);
                    self.entities[i] = entity;
                }
                self.tick_count += 1;
            }
        }
    }
}
