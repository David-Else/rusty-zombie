mod events;
mod hero;
mod movement;
mod random;
mod render;
mod types;
mod world;
mod zombie;
use crossterm::event::{self, Event, KeyCode};
use events::GameUI;
use render::{cleanup_terminal, setup_terminal};
use std::error::Error;
use std::time::{Duration, Instant};
use types::Direction;
use world::GameState;

fn main() -> Result<(), Box<dyn Error>> {
    // set a fixed frame duration for each 'tick' of the game loop
    let frame_duration = Duration::from_nanos(1_000_000_000u64 / 60); // 60 FPS

    // set the poll duration to zero for non-blocking input check
    let input_poll_duration = Duration::from_millis(0);

    // setup the terminal and return the resulting stdout and screensize depending on the window dimenstions
    let (stdout, screensize) = setup_terminal()?;

    // create game state
    let mut game_state = GameState::new(screensize);
    game_state.add_zombies(64);

    // add observers
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

    cleanup_terminal(stdout)?;
    Ok(())
}
