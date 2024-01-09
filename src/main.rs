mod events;
mod hero;
mod movement;
mod random;
mod render;
mod types;
mod world;
mod zombie;
use crossterm::event::{self, Event};
use events::{GameEvent, GameUI, InputObserver};
use render::Terminal;
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
    // let (stdout, screensize) = setup_terminal()?;
    let mut terminal = Terminal::new().expect("Failed to initialize terminal");
    let screensize = Terminal::screen_size().expect("Failed to initialize terminal");

    // create game state
    let mut game_state = GameState::new(screensize);
    game_state.add_zombies(64);

    // add observers
    let input_observer = InputObserver;
    game_state.register_observer(Box::new(GameUI));
    game_state.register_observer(Box::new(input_observer));

    // game loop
    while game_state.is_running {
        let loop_start = Instant::now(); // Mark the beginning of the loop iteration

        // Check for keyboard input
        if event::poll(input_poll_duration)? {
            if let Event::Key(key_event) = event::read()? {
                game_state.notify_observers(GameEvent::KeyPress(key_event.code));
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

    terminal.cleanup().expect("Failed to cleanup terminal");
    Ok(())
}
