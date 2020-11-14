use rustbox::{Color, RustBox};
use super::GameState;
use super::{Game,Position,Entity,Animal};

const SIDE_PANEL_WIDTH: usize = 32;

pub fn render(rustbox: &RustBox, game: &super::Game) {
    rustbox.clear();
    let camera_position = game.camera_position();
    render_map(rustbox, camera_position, &game.map);
    render_player(rustbox, camera_position, game.player_position());
    for i in 0..game.entities.len() {
        render_entity(rustbox, camera_position, &game.entities[i]);
    }
    if game.current_state() == GameState::InspectTiles {
        render_cursor(rustbox, camera_position)
    }
    render_side_panel(rustbox, game);
    rustbox.present();
}

fn game_coords_to_camera(rustbox: &RustBox, camera_position: &Position, position: &Position) -> (usize, usize) {
    let result_x = position.x() + (rustbox.width() as i32) / 2 - camera_position.x();
    let result_y = position.y() + (rustbox.height() as i32) / 2 - camera_position.y();
    if result_x < 0 || result_y < 0 {
        return (rustbox.width()+1, rustbox.height()+1)
    }
    (result_x as usize, result_y as usize)
}

fn is_visible(rustbox: &RustBox, x: usize, y: usize) -> bool {
    x < rustbox.width() && y < rustbox.height() && x > SIDE_PANEL_WIDTH
}

fn render_map(rustbox: &RustBox, camera_position: &Position, map: &super::GameMap) {
    let portion = map.portion_around(camera_position, 100);
    for i in 0..portion.len() {
        render_tile(rustbox, &portion[i], camera_position);
    }
}

fn render_tile(rustbox: &RustBox, tile: &super::Tile, camera_position: &Position) {
    let position = tile.position();
    let (x, y) = game_coords_to_camera(rustbox, camera_position, position);
    if is_visible(rustbox, x, y) {
        let bg_color = Color::Black;
        let fg_color = match &tile.tile_type {
            super::TileType::Grass => Color::Green,
            super::TileType::Dirt => Color::Yellow,
        };
        let graphic = match &tile.tile_type {
            super::TileType::Grass => match &tile.variant {
                1 => ",",
                2 => " ",
                3 => "'",
                4 => "\"",
                _ => " "
            },
            super::TileType::Dirt => match &tile.variant {
                1 => " ",
                2 => "-",
                3 => ".",
                4 => "_",
                _ => " "
            }
        };
        rustbox.print(x, y, rustbox::RB_NORMAL, fg_color, bg_color, graphic);
    }
}

fn render_player(rustbox: &RustBox, camera_position: &Position, player_position: &Position) {
    let (x, y) = game_coords_to_camera(rustbox, camera_position, player_position);
    if is_visible(rustbox, x, y) {
        rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, Color::Default, "@");
    }
}

fn render_entity(rustbox: &RustBox, camera_position: &Position, entity: &Animal) {
    let (x, y) = game_coords_to_camera(rustbox, camera_position, entity.current_position());
    if is_visible(rustbox, x, y) {
        let color = if entity.state == super::animals::AnimalState::Idle { Color::Cyan } else { Color::Red };
        rustbox.print(x, y, rustbox::RB_NORMAL, color, Color::Default, "a");
    }
}

fn render_cursor(rustbox: &RustBox, camera_position: &Position) {
    let (x, y) = game_coords_to_camera(rustbox, camera_position, camera_position);
    rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, Color::Default, "X");
}

fn render_side_panel(rustbox: &RustBox, game: &Game) {
    for i in 0..rustbox.width() {
        rustbox.print(SIDE_PANEL_WIDTH, i, rustbox::RB_NORMAL, Color::White, Color::Default, "|");
    }
    rustbox.print(0, 0, rustbox::RB_NORMAL, Color::White, Color::Default, "Player");
    render_debug(rustbox, game, 2);
}

fn render_debug(rustbox: &RustBox, game: &Game, y: usize) {
    rustbox.print(0, y, rustbox::RB_NORMAL, Color::White, Color::Default, "Debug Info:");
    rustbox.print(0, y + 1, rustbox::RB_NORMAL, Color::White, Color::Default, &format!("Ticks: {}", game.tick_count()).to_string());
    rustbox.print(0, y + 2, rustbox::RB_NORMAL, Color::White, Color::Default, &format!("Entity Count: {}", game.entities.len()).to_string());
    rustbox.print(0, y + 3, rustbox::RB_NORMAL, Color::White, Color::Default, &format!("Player Pos: {} {}", game.player_position().x(), game.player_position().y()).to_string());
    rustbox.print(0, y + 4, rustbox::RB_NORMAL, Color::White, Color::Default, &format!("Game Mode: {}", game.state.to_string()));
}