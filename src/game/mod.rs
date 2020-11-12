use rustbox::{Key, RustBox};
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::cmp::{min,max};

const MAP_SIZE: usize = 1000;

type RNG = Pcg64;

pub mod render;
pub mod animals;

pub trait Entity {
    fn position(&self) -> (i32, i32);
    fn move_relative(&mut self, h: i32, v: i32);
    fn tick(&mut self, game: &mut Game);

    fn can_see(&self, entity: &dyn Entity) -> bool {
        self.distance_from(entity) < 15.0
    }

    fn distance_parts_from(&self, entity: &dyn Entity) -> (i32, i32) {
        let dx = entity.position().0 - self.position().0;
        let dy = entity.position().1 - self.position().1;
        (dx, dy)
    }

    fn distance_from(&self, entity: &dyn Entity) -> f32 {
        let distance = self.distance_parts_from(entity);
        ((distance.0.pow(2) + distance.1.pow(2)) as f32).sqrt()
    }

    fn direction_for(&self, entity: &dyn Entity) -> (i32, i32) {
        let distance = self.distance_parts_from(entity);
        (max(-1, min(1, distance.0)), max(-1, min(1, distance.1)))
    }
}

#[derive(Copy, Clone)]
pub struct Player {
    x: i32,
    y: i32
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y
        }
    }
}

impl Entity for Player {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn move_relative(&mut self, h: i32, v: i32) {
        self.x += h;
        self.y += v;
    }

    fn tick(&mut self, game: &mut Game) {
        
    }
}

pub struct Game {
    player: Player,
    entities: Vec<animals::Animal>,
    rustbox: RustBox,
    map: GameMap,
    rng: RNG,
    tick_count: usize
}

#[derive(Copy, Clone)]
pub enum TileType {
    Dirt,
    Grass
}

#[derive(Copy, Clone)]
pub struct Tile {
    x: usize,
    y: usize,
    pub tile_type: TileType,
    pub variant: usize
}

impl Tile {
    pub fn new(x: usize, y: usize) -> Tile {
        Tile { tile_type: TileType::Dirt, variant: 0, x: x, y: y }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub struct GameMap {
    tiles: Vec<Tile>
}

impl GameMap {
    pub fn new() -> GameMap {
        GameMap { tiles: Vec::new() }
    }

    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < MAP_SIZE && (y as usize) < MAP_SIZE
    }

    pub fn at(&self, x: i32, y: i32) -> Option<&Tile> {
        if self.is_in_bounds(x, y) {
            Some(&self.tiles[MAP_SIZE * (x as usize) + (y as usize)])
        } else {
            None
        }
    }

    pub fn index_to_position(&self, i: usize) -> (usize, usize) {
        (i / MAP_SIZE, i % MAP_SIZE)
    }

    pub fn portion_around(&self, pos: (i32, i32), distance: usize) -> Vec<Tile> {
        let mut portion: Vec<Tile> = Vec::new();
        let range_x_min = max(0, pos.0 - distance as i32) as usize;
        let range_x_max = min(MAP_SIZE as i32, pos.0 + distance as i32) as usize;
        let range_y_min = max(0, pos.1 - distance as i32) as usize;
        let range_y_max = min(MAP_SIZE as i32, pos.1 + distance as i32) as usize;
        for x in range_x_min..range_x_max {
            for y in range_y_min..range_y_max {
                match self.at(x as i32, y as i32) {
                    Some(tile) => {
                        portion.push(tile.clone());
                    },
                    _ => ()
                }
            } 
        }
        portion
    }

    pub fn generate(&mut self, rng: &mut Pcg64) {
        let mut tiles = Vec::with_capacity(MAP_SIZE*MAP_SIZE);
        for i in 0..tiles.capacity() {
            let position = self.index_to_position(i);
            let mut tile = Tile::new(position.0, position.1);
            if rng.gen_bool(0.6) {
                tile.tile_type = TileType::Grass;
            }
            if rng.gen_bool(0.2) {
                tile.variant = rng.gen_range(1, 5);
            }
            tiles.push(tile);
        }
        self.tiles = tiles;
    }
}

impl Game {
    pub fn new() -> Game {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("Failed: {}", e),
        };
        let map_center = (MAP_SIZE as i32) / 2;
        let mut game = Game {
            player: Player::new(map_center, map_center),
            rustbox: rustbox,
            rng: Pcg64::seed_from_u64(1234),
            map: GameMap::new(),
            entities: Vec::new(),
            tick_count: 0
        };
        game.map.generate(&mut game.rng);
        for _ in 0..10 {
            let animal = animals::Animal::new(map_center + game.rng.gen_range(-50, 50), map_center + game.rng.gen_range(-50, 50));
            game.entities.push(animal);
        }
        game
    }

    pub fn player_position(&self) -> (i32, i32) {
        (self.player.x, self.player.y)
    }

    pub fn process_input(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('q') => {
                return false;
            }
            Key::Left => {
                self.player.move_relative(-1, 0)
            }
            Key::Right => {
                self.player.move_relative(1, 0)
            }
            Key::Up => {
                self.player.move_relative(0, -1)
            }
            Key::Down => {
                self.player.move_relative(0, 1)
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
