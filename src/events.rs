use crate::world::GameState;

pub enum GameEvent {
    HeroKilled,
    // Other game events...
}

// is this a good idea to have it here?
pub trait Observer {
    fn on_notify(&self, event: &GameEvent, game_state: &mut GameState);
}

pub struct GameUI;

// This observer is stateless so it does not need to be mutable, the struct is empty
// The GameState is mutable and is passed in, maybe we should only pass Entities?
impl Observer for GameUI {
    fn on_notify(&self, event: &GameEvent, game_state: &mut GameState) {
        match event {
            GameEvent::HeroKilled => {
                // Update UI to show player death (e.g., a game over screen)
                println!("Game Over!");
                println!(
                    "Hero is currently at position: {:?}",
                    game_state.hero.position
                );
            } // Handle any other UI-related events...
        }
    }
}
