use super::position::*;
use super::Game;
use super::render::RenderInfo;
use super::rpg::CharacterStats;

pub trait Entity {
    fn current_position(&self) -> &Position;
    fn position(&mut self) -> &mut Position;
    fn tick(&mut self, game: &mut Game);
    fn render_info(&self) -> &RenderInfo;
    fn name(&self) -> Option<&str>;
    fn kind(&self) -> &str;
    fn stats(&self) -> &CharacterStats;

    fn can_see(&self, position: &Position) -> bool {
        self.current_position().distance_from(position) < (self.stats().per.lvl() as f32)
    }
}