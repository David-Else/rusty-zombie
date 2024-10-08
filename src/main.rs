mod bullets;
mod events;
mod hero;
mod input;
mod movement;
mod random;
mod render;
mod types;
mod world;
mod zombie;
use events::GameUI;
use input::InputHandler;
use render::ConsoleRenderer;
use render::Renderer;
use std::error::Error;
use std::time::{Duration, Instant};
use world::GameLogic;
use world::GameState;

fn main() -> Result<(), Box<dyn Error>> {
    // set a fixed frame duration for each 'tick' of the game loop
    // 1000 milliseconds / 60 frames = approximately 16.67 milliseconds per frame
    const FPS_TARGET: u64 = 60;
    const NANOS_PER_SECOND: u64 = 1_000_000_000;
    const FRAME_DURATION: Duration = Duration::from_nanos(NANOS_PER_SECOND / FPS_TARGET);

    // setup the terminal
    let mut console_renderer = ConsoleRenderer::new();

    // create game state
    let mut game_state = GameState::new(console_renderer.screen_size());
    game_state.add_zombies(64);

    // add observers
    game_state.register_observer(Box::new(GameUI));

    // game loop
    while game_state.is_running {
        let loop_start = Instant::now(); // Mark the beginning of the loop iteration

        if let Some(input) = InputHandler::process_input() {
            game_state.handle_game_input(input);
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

        if loop_duration > FRAME_DURATION {
            println!("Warning: Frame rate dropped below {} FPS.", FPS_TARGET);
        }

        // If the loop finished faster than the frame duration, sleep the remaining time
        if loop_duration < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - loop_duration);
        }
    }

    console_renderer.cleanup();
    Ok(())
}
