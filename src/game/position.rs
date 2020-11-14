use std::cmp::{min,max};

#[derive(Copy, Clone)]
pub struct Position {
    x: i32,
    y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn move_relative(&mut self, h: i32, v: i32) {
        self.x = self.x + h; 
        self.y = self.y + v;
    }

    pub fn distance_parts_from(&self, position: &Position) -> (i32, i32) {
        let dx = position.x() - self.x;
        let dy = position.y() - self.y;
        (dx, dy)
    }

    pub fn distance_from(&self, position: &Position) -> f32 {
        let distance = self.distance_parts_from(position);
        ((distance.0.pow(2) + distance.1.pow(2)) as f32).sqrt()
    }

    pub fn direction_for(&self, position: &Position) -> (i32, i32) {
        let distance = self.distance_parts_from(position);
        (max(-1, min(1, distance.0)), max(-1, min(1, distance.1)))
    }
}