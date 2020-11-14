use rustbox::{Color, RustBox};
use super::Entity;
use super::Position;

pub fn render(rustbox: &RustBox, game: &super::Game) {
    rustbox.clear();
    /*
    rustbox.print(
        1,
        1,
        rustbox::RB_BOLD,
        Color::White,
        Color::Black,
        "Hello, world!",
    );
    rustbox.print(
        1,
        3,
        rustbox::RB_BOLD,
        Color::White,
        Color::Black,
        "Press 'q' to quit.",
    );
    */
    let player_position = game.player_position();
    render_map(rustbox, player_position, &game.map);
    render_player(rustbox, player_position);
    for i in 0..game.entities.len() {
        render_entity(rustbox, player_position, &game.entities[i]);
    }
    rustbox.present();
}

fn game_coords_to_camera(rustbox: &RustBox, player_position: &Position, position: &Position) -> (usize, usize) {
    let result_x = position.x() + (rustbox.width() as i32) / 2 - player_position.x();
    let result_y = position.y() + (rustbox.height() as i32) / 2 - player_position.y();
    if result_x < 0 || result_y < 0 {
        return (rustbox.width()+1, rustbox.height()+1)
    }
    (result_x as usize, result_y as usize)
}

fn is_visible(rustbox: &RustBox, x: usize, y: usize) -> bool {
    x < rustbox.width() && y < rustbox.height()
}

fn render_map(rustbox: &RustBox, player_position: &Position, map: &super::GameMap) {
    let portion = map.portion_around(player_position, 100);
    for i in 0..portion.len() {
        render_tile(rustbox, &portion[i], player_position);
    }
}

fn render_tile(rustbox: &RustBox, tile: &super::Tile, player_position: &Position) {
    let position = tile.position();
    let (x, y) = game_coords_to_camera(rustbox, player_position, position);
    if is_visible(rustbox, x, y) {
        let bg_color = Color::Black;
        let fg_color = match &tile.tile_type {
            super::TileType::Grass => Color::Green,
            super::TileType::Dirt => Color::Yellow,
            _ => Color::Black
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

fn render_player(rustbox: &RustBox, player_position: &Position) {
    let (x, y) = game_coords_to_camera(rustbox, player_position, player_position);
    if is_visible(rustbox, x, y) {
        rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, Color::Default, "@");
    }
}

fn render_entity(rustbox: &RustBox, player_position: &Position, entity: &super::animals::Animal) {
    let (x, y) = game_coords_to_camera(rustbox, player_position, entity.current_position());
    if is_visible(rustbox, x, y) {
        let color = if entity.state == super::animals::AnimalState::Idle { Color::Cyan } else { Color::Red };
        rustbox.print(x, y, rustbox::RB_NORMAL, color, Color::Default, "a");
    }
}