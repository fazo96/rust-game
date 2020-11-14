use super::position::*;
use super::Game;
use super::Entity;

#[derive(Copy, Clone)]
pub struct Player {
    position: Position
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            position: Position::new(x, y)
        }
    }
}

impl Entity for Player {
    fn current_position(&self) -> &Position {
        &self.position
    }

    fn position(&mut self) -> &mut Position {
        &mut self.position
    }

    fn tick(&mut self, game: &mut Game) {
        
    }
}