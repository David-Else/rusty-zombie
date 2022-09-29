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

    let (number_cols, number_rows) = terminal::size()?;
    let screensize = Point2d {
        x: number_rows as usize,
        y: number_cols as usize,
    };

    let mut game_state = GameState::new(&screensize);

    game_state.add_hero('h');
    game_state.add_zombies(64, 'z', number_cols as usize, number_rows as usize);

    game_state.render_screen(&stdout)?; // Delete, temp way to start with screen

    //Game loop
    'gameloop: loop {
        while event::poll(Duration::default())? {
            game_state.render_screen(&stdout)?;
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Char('h') => {
                        game_state.update("h");
                    }
                    KeyCode::Char('j') => {
                        game_state.update("j");
                    }
                    KeyCode::Char('k') => {
                        game_state.update("k");
                    }
                    KeyCode::Char('l') => {
                        game_state.update("l");
                    }
                    _ => {}
                }
            }
        }
    }

    // cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
