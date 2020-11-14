use super::position::*;
use super::{Game, GameState};
use super::Entity;
use super::render::RenderInfo;
use rustbox::{Color,Key};

#[derive(Copy, Clone)]
pub struct Player {
    position: Position,
    cursor_position: Position,
    render_info: RenderInfo
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            position: Position::new(x, y),
            cursor_position: Position::new(x, y),
            render_info: RenderInfo::new('@', Color::White)
        }
    }

    pub fn cursor_position(&self) -> &Position {
        &self.cursor_position
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

    fn tick(&mut self, game: &mut Game) {
        let position = match game.state {
            GameState::InspectTiles => &mut self.cursor_position,
            _ => &mut self.position
        };
        match game.last_input_key {
            None => {},
            Some(key) => match key {
                Key::Left => {
                    position.move_relative(-1, 0)
                }
                Key::Right => {
                    position.move_relative(1, 0)
                }
                Key::Up => {
                    position.move_relative(0, -1)
                }
                Key::Down => {
                    position.move_relative(0, 1)
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