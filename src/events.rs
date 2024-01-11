use std::{thread, time::Duration};

use crate::{
    types::Direction,
    world::{GameState, Screen},
};
use crossterm::event::KeyCode;

pub enum GameEvent {
    HeroKilled,
    KeyPress(KeyCode),
    // Other game events...
}

pub trait Observer {
    fn on_notify(&self, event: &GameEvent, game_state: &mut GameState);
}

// This observer is stateless so it does not need to be mutable, the struct is empty
// The GameState is mutable and is passed in, maybe we should only pass Entities?
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
            // hero is only updated on a key press, unlike other entities
            KeyCode::Char('k') => game_state.update_hero(Direction::Up),
            KeyCode::Char('j') => game_state.update_hero(Direction::Down),
            KeyCode::Char('h') => game_state.update_hero(Direction::Left),
            KeyCode::Char('l') => game_state.update_hero(Direction::Right),
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
