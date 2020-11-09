use rustbox::{Color, RustBox};

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
    render_map(rustbox, &game.player, &game.map);
    render_tree(rustbox, &game.player, (110, 110));
    render_player(rustbox, &game.player);
    rustbox.present();
}

fn game_coords_to_camera(rustbox: &RustBox, player: &super::Entity, x: i32, y: i32) -> (usize, usize) {
    let result_x = x + (rustbox.width() as i32) / 2 - player.x;
    let result_y = y + (rustbox.height() as i32) / 2 - player.y;
    if result_x < 0 || result_y < 0 {
        return (rustbox.width()+1, rustbox.height()+1)
    }
    (result_x as usize, result_y as usize)
}

fn is_visible(rustbox: &RustBox, x: usize, y: usize) -> bool {
    x < rustbox.width() && y < rustbox.height()
}

fn render_map(rustbox: &RustBox, player: &super::Entity, map: &[i32; 10000]) {
    for i in 0..map.len() {
        let (x, y) = game_coords_to_camera(rustbox, player, (i % 100) as i32, (i / 100) as i32);
        if is_visible(rustbox, x, y) {
            rustbox.print(x, y, rustbox::RB_NORMAL, Color::Yellow, Color::Yellow, " ");
        }
    }
}

fn render_tree(rustbox: &RustBox, player: &super::Entity, coords: (i32, i32)) {
    let (x, y) = game_coords_to_camera(rustbox, player, coords.0, coords.1);
    if is_visible(rustbox, x, y) {
        rustbox.print(x, y, rustbox::RB_NORMAL, Color::Green, Color::Default, "T");
    }
}

fn render_player(rustbox: &RustBox, player: &super::Entity) {
    let (x, y) = game_coords_to_camera(rustbox, player, player.x, player. y);
    if is_visible(rustbox, x, y) {
        rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, Color::Default, "@");
    }
}