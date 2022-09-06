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
    let screensize = Point2d { x: 16, y: 16 };
    let mut game_state = GameState::new(&screensize);

    game_state.add_hero('h');
    game_state.add_zombie('z');
    game_state.render_screen();

    loop {
        println!("Press hjkl to move the hero");

        let mut key = String::new();

        io::stdin()
            .read_line(&mut key)
            .expect("Failed to read line");

        game_state.update(key.trim());
        game_state.render_screen();
    }
}
