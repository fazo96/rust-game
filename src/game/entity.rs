use super::position::*;
use super::Game;

pub trait Entity {
    fn current_position(&self) -> &Position;
    fn position(&mut self) -> &mut Position;
    fn tick(&mut self, game: &mut Game);

    fn can_see(&self, entity: &dyn Entity) -> bool {
        self.current_position().distance_from(entity.current_position()) < 15.0
    }
}