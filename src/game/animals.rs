use rand::prelude::*;
use super::Entity;
use super::position::*;

#[derive(Copy, Clone, PartialEq)]
pub enum AnimalState {
    Idle,
    FleeFromPlayer
}

#[derive(Copy, Clone)]
pub struct Animal {
    position: Position,
    pub state: AnimalState
}

impl Animal {
    pub fn new(x: i32, y: i32) -> Animal {
        Animal {
            position: Position::new(x, y),
            state: AnimalState::Idle
        }
    }
}

impl Entity for Animal {
    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn current_position(&self) -> &Position {
        &self.position
    }

    fn tick(&mut self, game: &mut super::Game) {
        match self.state {
            AnimalState::Idle => {
                if game.rng.gen_bool(0.3) {
                    // Move randomly, sometimes.
                    self.position.move_relative(game.rng.gen_range(-1, 2), game.rng.gen_range(-1, 2))
                }
                if self.can_see(&game.player) {
                    // If we see the player we flee
                    self.state = AnimalState::FleeFromPlayer;
                } 
            },
            AnimalState::FleeFromPlayer => {
                if self.can_see(&game.player) {
                    // If we see the player we flee
                    let dir = self.current_position().direction_for(game.player_position());
                    self.position().move_relative(-1 * dir.0, -1 * dir.1)
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