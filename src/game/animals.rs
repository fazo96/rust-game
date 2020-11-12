use rand::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum AnimalState {
    Idle,
    FleeFromPlayer
}

#[derive(Copy, Clone)]
pub struct Animal {
    x: i32,
    y: i32,
    pub state: AnimalState
}

impl Animal {
    pub fn new(x: i32, y: i32) -> Animal {
        Animal {
            x,
            y,
            state: AnimalState::Idle
        }
    }
}

impl super::Entity for Animal {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn move_relative(&mut self, h: i32, v: i32) {
        self.x += h;
        self.y += v;
    }

    fn tick(&mut self, game: &mut super::Game) {
        match self.state {
            AnimalState::Idle => {
                if game.rng.gen_bool(0.3) {
                    // Move randomly, sometimes.
                    self.move_relative(game.rng.gen_range(-1, 2), game.rng.gen_range(-1, 2))
                }
                if self.can_see(&game.player) {
                    // If we see the player we flee
                    self.state = AnimalState::FleeFromPlayer;
                } 
            },
            AnimalState::FleeFromPlayer => {
                if self.can_see(&game.player) {
                    // If we see the player we flee
                    let dir = self.direction_for(&game.player);
                    self.move_relative(-1 * dir.0, -1 * dir.1)
                } else {
                    // If we don't see it anymore, maybe we stop fleeing
                    if game.rng.gen_bool(0.3) {
                        self.state = AnimalState::Idle;
                    }
                }
            }
        }
    }
}