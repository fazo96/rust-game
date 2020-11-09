use rustbox::Key;

pub mod render;

pub struct Entity {
    pub x: usize,
    pub y: usize,
}

pub struct Game {
    pub player: Entity,
}

pub fn process_input(key: &Key, game: &mut Game) -> bool {
    match key {
        Key::Char('q') => {
            return false;
        }
        Key::Left => {
            game.player.x -= 1;
        }
        Key::Right => {
            game.player.x += 1;
        }
        Key::Up => {
            game.player.y -= 1;
        }
        Key::Down => {
            game.player.y += 1;
        }
        _ => {}
    }
    return true;
}