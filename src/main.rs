mod hero;
mod world;
mod zombie;
use std::io;
use world::GameState;

#[derive(Debug)]
// a position on a grid to be displayed on the terminal
pub struct Point2d {
    pub x: usize,
    pub y: usize,
}

fn main() {
    let screensize = Point2d { x: 32, y: 32 };
    let mut game_state = GameState::new(&screensize);

    game_state.add_hero('h');
    game_state.add_zombies(32, 'z');

    loop {
        game_state.render_screen();
        println!("Press hjkl to move the hero");

        let mut key = String::new();

        io::stdin()
            .read_line(&mut key)
            .expect("Failed to read line");

        game_state.update(key.trim());
    }
}
