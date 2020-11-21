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
pub mod rpg;

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
    player: Box<Player>,
    entities: Vec<Box<dyn Entity>>,
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
            player: Box::new(Player::new(map_center, map_center)),
            rustbox,
            rng: Pcg64::seed_from_u64(1234),
            map,
            entities: Vec::new(),
            tick_count: 0,
            last_input_key: None
        };
        // Generate Map
        game.map.generate(&mut game.rng);
        // Populate Entities
        for _ in 0..10 {
            let animal = Animal::new(map_center + game.rng.gen_range(-50, 50), map_center + game.rng.gen_range(-50, 50));
            game.entities.push(Box::new(animal));
        }
        game
    }

    pub fn current_state(&self) -> GameState {
        self.state
    }

    pub fn player_position(&self) -> &Position {
        self.player.current_position()
    }

    pub fn cursor_position(&self) -> &Position {
        self.player.cursor_position()
    }

    pub fn camera_position(&self) -> &Position {
        match self.state {
            GameState::InspectTiles => self.cursor_position(),
            _ => self.player_position()
        }
    }

    pub fn process_input(&mut self, key: &Key) {
        self.last_input_key = Some(key.clone());
    }

    pub fn tick_count(&self) -> usize {
        self.tick_count
    }

    pub fn entities_at(&self, position: &Position) -> Vec<&Box<dyn Entity>> {
        self.entities.iter().filter(|entity| entity.current_position() == position).collect()
    }

    pub fn tile_at(&self, position: &Position) -> Option<&Tile> {
        self.map.at(position.x(), position.y())
    }

    pub fn is_passable(&self, position: &Position) -> bool {
        let tile = self.tile_at(position);
        if tile.is_none() {
            false
        } else {
            let no_entities = self.entities_at(position).is_empty() && self.player.current_position() != position;
            tile.unwrap().is_passable() && no_entities
        }
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
            let mut player = *self.player.clone();
            player.tick(self);
            self.player = Box::new(player);
            if self.state == GameState::Gameplay {
                let len = self.entities.len();
                for i in 0..len {
                    let mut entity = self.entities.pop().unwrap();
                    entity.tick(self);
                    self.entities.push(entity);
                    self.entities.swap(i, len - 1)
                }
                self.tick_count += 1;
            }
        }
    }
}
