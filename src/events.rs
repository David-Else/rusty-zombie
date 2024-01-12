use crate::{
    types::Direction,
    world::{GameEvent, GameState, Screen},
};
use crossterm::event::KeyCode;

// Note: observers are mutable and pass in mutable game state, maybe only pass in entites?
// to make it immutable would be a massive change
pub trait Observer {
    fn on_notify(&self, event: &GameEvent, game_state: &mut GameState);
}

pub struct GameUI;
impl Observer for GameUI {
    fn on_notify(&self, event: &GameEvent, game_state: &mut GameState) {
        match event {
            GameEvent::HeroKilled => {
                game_state.current_screen = Screen::GameOver;
            }
            GameEvent::KeyPress(_) => {} // Handle any other UI-related events...
        }
    }
}

pub struct InputObserver;
impl InputObserver {
    // These are private methods for handling specific key presses on different screens.
    fn handle_start_menu_keys(&self, key_code: KeyCode, game_state: &mut GameState) {
        match key_code {
            KeyCode::Char('q') => {
                game_state.is_running = false;
            }
            KeyCode::Char('s') => {
                println!("Starting the game...");
                game_state.current_screen = Screen::GamePlay;
            }
            _ => {}
        }
    }

    fn handle_gameplay_keys(&self, key_code: KeyCode, game_state: &mut GameState) {
        match key_code {
            KeyCode::Char('q') => {
                game_state.is_running = false;
            }
            KeyCode::Char('k') => game_state
                .hero
                .move_in_direction(Direction::Up, game_state.screen_size),
            KeyCode::Char('j') => game_state
                .hero
                .move_in_direction(Direction::Down, game_state.screen_size),
            KeyCode::Char('h') => game_state
                .hero
                .move_in_direction(Direction::Left, game_state.screen_size),
            KeyCode::Char('l') => game_state
                .hero
                .move_in_direction(Direction::Right, game_state.screen_size),
            KeyCode::Char('f') => game_state.add_bullet(),
            _ => {} // Do nothing for all other keys
        }
    }
    fn handle_game_over_keys(&self, key_code: KeyCode, game_state: &mut GameState) {
        match key_code {
            KeyCode::Char('q') => {
                game_state.is_running = false;
            }
            KeyCode::Char('s') => {
                println!("Starting the game...");
                game_state.current_screen = Screen::GamePlay;
            }
            _ => {}
        }
    }
}

impl Observer for InputObserver {
    fn on_notify(&self, event: &GameEvent, game_state: &mut GameState) {
        match event {
            GameEvent::KeyPress(key_code) => match game_state.current_screen {
                Screen::StartMenu => self.handle_start_menu_keys(*key_code, game_state),
                Screen::GamePlay => self.handle_gameplay_keys(*key_code, game_state),
                Screen::GameOver => self.handle_game_over_keys(*key_code, game_state),
            },
            _ => {}
        }
    }
}
