mod bullets;
mod events;
mod hero;
mod movement;
mod random;
mod render;
mod types;
mod world;
mod zombie;
use crossterm::event::{self, Event};
use events::{GameUI, InputObserver};
use render::ConsoleRenderer;
use render::Renderer;
use std::error::Error;
use std::time::{Duration, Instant};
use world::GameEvent;
use world::GameLogic;
use world::GameState;

fn main() -> Result<(), Box<dyn Error>> {
    // set a fixed frame duration for each 'tick' of the game loop
    let frame_duration = Duration::from_nanos(1_000_000_000u64 / 60); // 60 FPS

    // set the poll duration to zero for non-blocking input check
    let input_poll_duration = Duration::from_millis(0);

    // setup the terminal
    let mut console_renderer = ConsoleRenderer::new();

    // create game state
    let mut game_state = GameState::new(console_renderer.screen_size());
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

        game_state.update_state();
        game_state.check_collisions();
        console_renderer.render(
            &game_state.zombies,
            &game_state.bullets,
            &game_state.hero,
            &game_state.current_screen,
        )?;

        // Calculate how long the loop iteration took
        let loop_duration = loop_start.elapsed();

        // If the loop finished faster than the frame duration, sleep the remaining time
        if loop_duration < frame_duration {
            std::thread::sleep(frame_duration - loop_duration);
        }
    }

    console_renderer.cleanup();
    Ok(())
}
