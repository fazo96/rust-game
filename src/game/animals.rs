use rand::prelude::*;

pub struct Animal {
    pub x: i32,
    pub y: i32
}

impl super::Entity for Animal {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn move_relative(&mut self, h: i32, v: i32) {
        self.x += h;
        self.y += v;
    }

    fn tick(&mut self) {
      // self.move_relative(game.rng.gen_range(-1, 1), game.rng.gen_range(-1, 1))
    }
}