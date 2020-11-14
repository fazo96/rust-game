use super::position::*;
use super::Game;
use super::render::RenderInfo;

pub trait Entity {
    fn current_position(&self) -> &Position;
    fn position(&mut self) -> &mut Position;
    fn tick(&mut self, game: &mut Game);
    fn render_info(&self) -> &RenderInfo;

    fn can_see(&self, entity: &dyn Entity) -> bool {
        self.current_position().distance_from(entity.current_position()) < 15.0
    }
}