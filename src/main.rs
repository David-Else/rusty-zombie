mod hero;
mod world;
mod zombie;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{error::Error, io, time::Duration};
use world::GameState;

#[derive(Debug)]
// a position on a grid to be displayed on the terminal
pub struct Point2d {
    pub x: usize,
    pub y: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let screensize = Point2d { x: 32, y: 32 };
    let mut game_state = GameState::new(&screensize);

    game_state.add_hero('h');
    game_state.add_zombies(64, 'z');

    game_state.render_screen();

    //Game loop
    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    // KeyCode::Char('l') => {
                    //     break 'gameloop;
                    // }
                    _ => {
                        // hello
                        // game_state.update(key_event.code.unwrap());
                        println!("{:?}", key_event.code)
                    }
                }
            }
        }
    }

    // cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())

    // loop {
    //     game_state.render_screen();
    //     println!("Press hjkl to move the hero");

    //     let mut key = String::new();

    //     io::stdin()
    //         .read_line(&mut key)
    //         .expect("Failed to read line");

    //     game_state.update(key.trim());
    // }
}
