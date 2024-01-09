mod hero;
mod movement;
mod random;
mod render;
mod world;
mod zombie;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, size, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::time::{Duration, Instant};
use std::{error::Error, io};
use world::{GameState, GameUI};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
    pub x: usize,
    pub y: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    // set a fixed frame duration for each 'tick' of the game loop
    let frame_duration = Duration::from_nanos(1_000_000_000u64 / 60); // 60 FPS

    // set the poll duration to zero for non-blocking input check
    let input_poll_duration = Duration::from_millis(0);

    // setup terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    let screensize = {
        let (number_cols, number_rows) = size()?;
        Point2d {
            x: number_rows as usize,
            y: number_cols as usize,
        }
    };

    // create game state
    let mut game_state = GameState::new(screensize);
    game_state.add_zombies(64);

    game_state.register_observer(Box::new(GameUI));

    'gameloop: loop {
        let loop_start = Instant::now(); // Mark the beginning of the loop iteration

        if event::poll(input_poll_duration)? {
            // If there's an event, process it
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Char('h') => {
                        game_state.update_hero(Direction::Left);
                    }
                    KeyCode::Char('j') => {
                        game_state.update_hero(Direction::Down);
                    }
                    KeyCode::Char('k') => {
                        game_state.update_hero(Direction::Up);
                    }
                    KeyCode::Char('l') => {
                        game_state.update_hero(Direction::Right);
                    }
                    _ => {}
                }
            }
        }

        // Execute the tick function to render/update the game and check physics
        game_state.tick()?;

        // Calculate how long the loop iteration took
        let loop_duration = loop_start.elapsed();
        // If the loop finished faster than the frame duration, sleep the remaining time
        if loop_duration < frame_duration {
            std::thread::sleep(frame_duration - loop_duration);
        }
    }

    // cleanup terminal
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
