use rustbox::{Key, RustBox};
use rand::prelude::*;
use rand_pcg::Pcg64;

const MAP_SIZE: usize = 100;

type RNG = Pcg64;
type GameMap = Vec<Vec<Tile>>;

pub mod render;
pub mod animals;

trait Entity {
    fn position(&self) -> (i32, i32);
    fn move_relative(&mut self, h: i32, v: i32);
    fn tick(&mut self);
}

pub struct Player {
    pub x: i32,
    pub y: i32
}

impl Entity for Player {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn move_relative(&mut self, h: i32, v: i32) {
        self.x += h;
        self.y += v;
    }

    fn tick(&mut self) {
        
    }
}

pub struct Game {
    pub player: Player,
    entities: Vec<animals::Animal>,
    rustbox: RustBox,
    map: GameMap,
    rng: RNG
}

pub enum TileType {
    Dirt,
    Grass
}

pub struct Tile {
    pub tile_type: TileType,
    pub variant: usize
}

impl Tile {
    pub fn new() -> Tile {
        Tile { tile_type: TileType::Dirt, variant: 0 }
    }
}

impl Game {
    pub fn new() -> Game {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("Failed: {}", e),
        };
        let player = Player { x: 30, y: 30 };
        let rng = Pcg64::seed_from_u64(1234);
        let mut game = Game {
            player: player,
            rustbox: rustbox,
            rng: rng,
            map: Vec::new(),
            entities: Vec::new()
        };
        game.generate_empty_map();
        game
    }

    fn generate_empty_map(&mut self) {
        let mut rows = Vec::with_capacity(MAP_SIZE);
        for _ in 0..rows.capacity() {
            let mut tiles = Vec::with_capacity(MAP_SIZE);
            for _ in 0..tiles.capacity() {
                let mut tile = Tile::new();
                if self.rng.gen_bool(0.6) {
                    tile.tile_type = TileType::Grass;
                }
                if self.rng.gen_bool(0.2) {
                    tile.variant = self.rng.gen_range(1, 5);
                }
                tiles.push(tile);
            }
            rows.push(tiles);
        }
        self.map = rows;
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
            self.player.tick();
            for i in 0..self.entities.len() {
                self.entities[i].tick()
            }
        }
    }
}
