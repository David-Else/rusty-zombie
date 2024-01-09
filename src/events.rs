use crate::world::GameState;

pub enum GameEvent {
    HeroKilled,
    // Other game events...
}

// is this a good idea to have it here?
pub trait Observer {
    fn on_notify(&self, event: &GameEvent, game_state: &mut GameState);
}
