use super::position::*;
use super::{Game, GameState};
use super::Entity;
use super::render::RenderInfo;
use super::rpg::CharacterStats;
use rustbox::{Color,Key};

#[derive(Clone)]
pub struct Player {
    position: Position,
    cursor_position: Position,
    render_info: RenderInfo,
    character_stats: CharacterStats
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            character_stats: CharacterStats::new(5, 5, 30),
            position: Position::new(x, y),
            cursor_position: Position::new(x, y),
            render_info: RenderInfo::new('@', Color::White)
        }
    }

    pub fn cursor_position(&self) -> &Position {
        &self.cursor_position
    }

    fn mov(&mut self, h: i32, v: i32, game: &Game) -> bool {
        let position = match game.state {
            GameState::InspectTiles => &mut self.cursor_position,
            _ => &mut self.position
        };
        match game.current_state() {
            GameState::Gameplay => {
                position.move_relative_if_passable(h, v, game)
            },
            GameState::InspectTiles => {
                position.move_relative(h, v);
                true
            },
            _ => false
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

    fn render_info(&self) -> &RenderInfo {
        &self.render_info
    }

    fn name(&self) -> Option<&str> {
        Some("Player")
    }

    fn kind(&self) -> &str {
        "Human"
    }

    fn stats(&self) -> &CharacterStats {
        &self.character_stats
    }

    fn tick(&mut self, game: &mut Game) {
        match game.last_input_key {
            None => {},
            Some(key) => match key {
                Key::Left => {
                    self.mov(-1, 0, game);
                }
                Key::Right => {
                    self.mov(1, 0, game);
                }
                Key::Up => {
                    self.mov(0, -1, game);
                }
                Key::Down => {
                    self.mov(0, 1, game);
                },
                Key::Esc => {
                    if game.state == GameState::Gameplay {
                        game.state = GameState::Quit;
                    } else {
                        game.state = GameState::Gameplay;
                    }
                },
                Key::Char('v') => {
                    game.state = match game.state {
                        GameState::InspectTiles => GameState::Gameplay,
                        _ => {
                            self.cursor_position = self.position.clone();
                            GameState::InspectTiles
                        }
                    }
                }
                _ => {}
            }
        }
    }
}