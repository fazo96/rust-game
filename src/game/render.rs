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
    render_tree(rustbox, 10, 10);
    render_player(rustbox, &game.player);
    rustbox.present();
}

fn render_tree(rustbox: &RustBox, x: usize, y: usize) {
    rustbox.print(x, y, rustbox::RB_NORMAL, Color::Green, Color::Default, "T");
}

fn render_player(rustbox: &RustBox, player: &super::Entity) {
    rustbox.print(player.x, player.y, rustbox::RB_NORMAL, Color::White, Color::Default, "@");
}