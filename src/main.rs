mod hero;
mod world;
mod zombie;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    style,
    terminal::{self, size, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::time::{Duration, Instant};
use std::{error::Error, io};
use world::GameState;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Nokey,
}

#[derive(Debug, Clone, Copy)]
// a position on a grid to be displayed on the terminal
pub struct Point2d {
    pub x: usize,
    pub y: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Set a fixed frame duration for each 'tick' of the game loop
    let frame_duration = Duration::from_nanos(1_000_000_000u64 / 60); // 60 FPS

    // Set the poll duration to zero for non-blocking input check
    let input_poll_duration = Duration::from_millis(0);

    // setup terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // get size of terminal
    let (number_cols, number_rows) = size()?;
    let screensize = Point2d {
        x: number_rows as usize,
        y: number_cols as usize,
    };

    // create game state
    let mut game_state = GameState::new(screensize);
    game_state.add_hero();
    game_state.add_zombies(64);

    // render screen
    game_state.render_screen(&stdout)?; // Delete, temp way to start with screen

    fn tick(game_state: &mut GameState) -> Result<(), Box<dyn Error>> {
        // Perform automatic game updates here, e.g., move zombies
        // game_state.move_zombies(); // A hypothetical method you'll need to implement
        // WARNING hack!!! sending a direction when it is not needed
        game_state.update_zombie(Direction::Up); // Check for collisions or any other periodic logic
                                                 // ...

        // Render the updated game state
        let mut stdout = io::stdout();

        game_state.render_screen(&mut stdout)?;

        if game_state.detect_zombie_collision_hero() {
            world::GameState::print_top_right(&mut stdout, "You are dead!")?;
            // stdout.queue(style::Print("YOU ARE DEAD!".to_string()))?;
        }
        Ok(())
    }

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

        // Execute the tick function to render the game
        tick(&mut game_state)?;

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
