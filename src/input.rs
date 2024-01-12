// src/input.rs
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub enum GameInput {
    Exit,
    Start,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Fire,
    // ... other inputs
}

pub struct InputHandler;

impl InputHandler {
    pub fn process_input() -> Option<GameInput> {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Ok(Event::Key(key_event)) = event::read() {
                return match key_event.code {
                    KeyCode::Char('q') => Some(GameInput::Exit),
                    KeyCode::Char('s') => Some(GameInput::Start),
                    KeyCode::Char('k') => Some(GameInput::MoveUp),
                    KeyCode::Char('j') => Some(GameInput::MoveDown),
                    KeyCode::Char('h') => Some(GameInput::MoveLeft),
                    KeyCode::Char('l') => Some(GameInput::MoveRight),
                    KeyCode::Char('f') => Some(GameInput::Fire),
                    _ => None,
                };
            }
        }
        None
    }
}
