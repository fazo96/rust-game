use rustbox::{Color, RustBox};
use super::GameState;
use super::{Game,Position,Entity};
use super::rpg::CharacterStats;

const SIDE_PANEL_WIDTH: usize = 32;

#[derive(Copy, Clone, PartialEq)]
pub struct RenderInfo {
    pub character: char,
    pub color: Color
}

impl RenderInfo {
    pub fn new(character: char, color: Color) -> RenderInfo {
        RenderInfo { character, color }
    }
}

pub fn render(rustbox: &RustBox, game: &super::Game) {
    rustbox.clear();
    let camera_position = game.camera_position();
    render_map(rustbox, camera_position, &game.player, &game.map);
    render_player(rustbox, camera_position, game.player_position());
    for i in 0..game.entities.len() {
        let entity = &*game.entities[i];
        if game.player.can_see(entity.current_position()) {
            render_entity(rustbox, camera_position, entity);
        }
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

fn render_map(rustbox: &RustBox, camera_position: &Position, player: &super::Player, map: &super::GameMap) {
    let portion = map.portion_around(camera_position, 100);
    for i in 0..portion.len() {
        let tile = &portion[i];
        if player.can_see(tile.position()) {
            render_tile(rustbox, tile, camera_position);
        }
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

fn render_entity(rustbox: &RustBox, camera_position: &Position, entity: &dyn Entity) {
    let (x, y) = game_coords_to_camera(rustbox, camera_position, entity.current_position());
    if is_visible(rustbox, x, y) {
        rustbox.print(x, y, rustbox::RB_NORMAL, entity.render_info().color, Color::Default, &entity.render_info().character.to_string());
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
    let mut y = render_entity_info(rustbox, game, 0);
    y = y + render_tile_info(rustbox, game, y);
    if y > 0 { y = y + 1 }
    render_debug(rustbox, game, y);
}

fn render_entity_info(rustbox: &RustBox, game: &Game, y: usize) -> usize {
    let position = game.camera_position();
    let optional_entity = game.entities_at(position).pop();
    let mut lines = 0;
    if optional_entity.is_some() {
        let entity = optional_entity.unwrap();
        let name = entity.name();
        let mut x = 0;
        if name.is_some() {
            x = name.unwrap().len() + 1;
            rustbox.print(0, y + lines, rustbox::RB_BOLD, Color::White, Color::Default, name.unwrap());
            lines += 1;
        }
        if x > 0 {
            rustbox.print(x, y + lines, rustbox::RB_NORMAL, Color::White, Color::Default, &format!(", {}", entity.kind()));
        } else {
            rustbox.print(x, y + lines, rustbox::RB_NORMAL, Color::White, Color::Default, entity.kind());
        }
        lines += 2;
        lines += render_character_stats(rustbox, entity.stats(), lines);
    } else if game.player_position() == position {
        let x = game.player.name().unwrap().len();
        rustbox.print(0, y + lines, rustbox::RB_BOLD, Color::White, Color::Default, game.player.name().unwrap());
        rustbox.print(x, y + lines, rustbox::RB_NORMAL, Color::White, Color::Default, &format!(", {}", game.player.kind()));
        lines += 2;
        lines += render_character_stats(rustbox, game.player.stats(), lines);
    }
    lines + 1
}

fn render_character_stats(rustbox: &RustBox, stats: &CharacterStats, y: usize) -> usize {
    let mut lines = 0;
    for stat in stats.as_vec() {
        let mut x = 0;
        let name = stat.name();
        let fg_color = match name {
            "Strength" => Color::Red,
            "Dexterity" => Color::Yellow,
            "Perception" => Color::Blue,
            _ => Color::White
        };
        rustbox.print(x, y + lines, rustbox::RB_NORMAL, fg_color, Color::Default, name);
        x = x + name.len();
        rustbox.print(x, y + lines, rustbox::RB_NORMAL, Color::White, Color::Default, &format!(": {}", stat.lvl()));
        lines = lines + 1;
    }
    lines
}

fn render_tile_info(rustbox: &RustBox, game: &Game, y: usize) -> usize {
    let tile = game.tile_at(game.camera_position());
    let mut lines = 0;
    let mut x = 0;
    if tile.is_some() {
        let fg_color = match tile.unwrap().tile_type {
            super::TileType::Grass => Color::Green,
            super::TileType::Dirt => Color::Yellow,
        };
        let name = tile.unwrap().name();
        rustbox.print(x, y + lines, rustbox::RB_NORMAL, fg_color, Color::Default, name);
        x = x + name.len();
        let passable = tile.unwrap().is_passable() && game.is_passable(game.camera_position());
        let passability = if passable { "Passable" } else { "Blocked" };
        rustbox.print(x, y + lines, rustbox::RB_NORMAL, Color::White, Color::Default, &format!(", {}", passability));
        lines = lines + 1;
    }
    lines
}

fn render_debug(rustbox: &RustBox, game: &Game, y: usize) -> usize {
    rustbox.print(0, y, rustbox::RB_NORMAL, Color::White, Color::Default, "Debug Info:");
    rustbox.print(0, y + 1, rustbox::RB_NORMAL, Color::White, Color::Default, &format!("Ticks: {}", game.tick_count()).to_string());
    rustbox.print(0, y + 2, rustbox::RB_NORMAL, Color::White, Color::Default, &format!("Entity Count: {}", game.entities.len()).to_string());
    rustbox.print(0, y + 3, rustbox::RB_NORMAL, Color::White, Color::Default, &format!("Player Pos: {} {}", game.player_position().x(), game.player_position().y()).to_string());
    rustbox.print(0, y + 4, rustbox::RB_NORMAL, Color::White, Color::Default, &format!("Game Mode: {}", game.state.to_string()));
    y + 5
}