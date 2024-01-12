use crate::world::{GameEvent, GameState, Screen};

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
        }
    }
}
