mod hero;
mod world;
mod zombie;
use std::io;
use world::GameState;

#[derive(Debug)]
pub struct Point2d {
    pub x: usize,
    pub y: usize,
}

fn main() {
    let mut game_state = GameState::new();
    game_state.add_hero(7, 7, 'h');
    game_state.add_zombie('z');
    game_state.render_screen();

    loop {
        println!("Press hjkl to move the hero");

        let mut key = String::new();

        io::stdin()
            .read_line(&mut key)
            .expect("Failed to read line");

        game_state.heroes[0].keys(key.trim().to_string());
        game_state.render_screen();
    }
}
