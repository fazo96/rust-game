use rand::prelude::*;
use super::Entity;
use super::position::*;
use super::render::RenderInfo;
use super::rpg::CharacterStats;
use rustbox::Color;

#[derive(Copy, Clone, PartialEq)]
pub enum AnimalState {
    Idle,
    FleeFromPlayer
}

#[derive(Clone)]
pub struct Animal {
    position: Position,
    state: AnimalState,
    render_info: RenderInfo,
    character_stats: CharacterStats
}

impl Animal {
    pub fn new(x: i32, y: i32) -> Animal {
        Animal {
            character_stats: CharacterStats::new(3, 10, 15),
            position: Position::new(x, y),
            state: AnimalState::Idle,
            render_info: RenderInfo::new('a', Color::Cyan)
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

    fn render_info(&self) -> &RenderInfo {
        &self.render_info
    }

    fn name(&self) -> Option<&str> {
        None
    }

    fn kind(&self) -> &str {
        "Animal"
    }

    fn stats(&self) -> &CharacterStats {
        &self.character_stats
    }

    fn tick(&mut self, game: &mut super::Game) {
        match self.state {
            AnimalState::Idle => {
                self.render_info.color = Color::Cyan;
                if game.rng.gen_bool(0.3) {
                    // Move randomly, sometimes.
                    self.position.move_relative_if_passable(game.rng.gen_range(-1, 2), game.rng.gen_range(-1, 2), game);
                }
                if self.can_see(&game.player.current_position()) {
                    // If we see the player we flee
                    self.state = AnimalState::FleeFromPlayer;
                } 
            },
            AnimalState::FleeFromPlayer => {
                self.render_info.color = Color::Red;
                if self.can_see(&game.player.current_position()) {
                    // If we see the player we flee
                    let dir = self.current_position().direction_for(game.player_position());
                    self.position.move_relative_if_passable(-1 * dir.0, -1 * dir.1, game);
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