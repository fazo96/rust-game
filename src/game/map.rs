use std::cmp::{min,max};
use super::RNG;
use rand::Rng;
use super::Position;

const MAP_SIZE: usize = 1000;

#[derive(Copy, Clone)]
pub enum TileType {
    Dirt,
    Grass
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub variant: usize,
    position: Position,
    passable: bool
}

impl Tile {
    pub fn new(x: usize, y: usize) -> Tile {
        Tile {
            tile_type: TileType::Dirt,
            variant: 0,
            position: Position::new(x as i32, y as i32),
            passable: true
        }
    }

    pub fn position(&self) -> &Position {
        &self.position 
    }

    pub fn is_passable(&self) -> bool {
        true
    }

    pub fn name(&self) -> &str {
        match self.tile_type {
            TileType::Grass => "Grass",
            TileType::Dirt => "Dirt"
        }
    }
}

pub struct GameMap {
    tiles: Vec<Tile>
}

impl GameMap {
    pub fn new() -> GameMap {
        GameMap { tiles: Vec::new() }
    }

    pub fn center() -> usize {
        MAP_SIZE / 2 
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

    pub fn portion_around(&self, pos: &Position, distance: usize) -> Vec<Tile> {
        let mut portion: Vec<Tile> = Vec::new();
        let range_x_min = max(0, pos.x() - distance as i32) as usize;
        let range_x_max = min(MAP_SIZE as i32, pos.x() + distance as i32) as usize;
        let range_y_min = max(0, pos.y() - distance as i32) as usize;
        let range_y_max = min(MAP_SIZE as i32, pos.y() + distance as i32) as usize;
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

    pub fn generate(&mut self, rng: &mut RNG) {
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