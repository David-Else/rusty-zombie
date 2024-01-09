use crate::{
    events::{GameEvent, Observer},
    hero::Hero,
    random::random_position_around_point,
    render::render_screen,
    types::{Entity, Point2d},
    zombie::Zombie,
    Direction,
};
use std::io::{self, Result};

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

// #[derive(Debug)]
pub struct GameState {
    // fields representing the state of the game
    pub zombies: Vec<Zombie>,
    pub hero: Hero,
    pub screen_size: Point2d,
    observers: Vec<Box<dyn Observer>>,
}

impl GameState {
    // methods to manage the game state
    pub fn new(screen_size: Point2d) -> Self {
        Self {
            zombies: Vec::new(), // The compiler knows that this vector is meant to hold elements of type `Zombie` variable
            hero: Hero::new(Point2d {
                x: screen_size.x / 2,
                y: screen_size.y / 2,
            }),
            screen_size,
            observers: vec![],
        }
    }

    pub fn register_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }
    // Rust's rules prevent you from calling a method with `&mut self` while iterating over a collection of references (`&self.observers`).
    // There's a rule that you cannot have multiple mutable references to the same data
    pub fn notify_observers(&mut self, event: GameEvent) {
        // Temporarily take ownership of observers using std::mem::take,
        // which replaces self.observers with an empty vector
        // and gives us the original vector to iterate over.
        let mut observers = std::mem::take(&mut self.observers);

        for observer in observers.iter_mut() {
            observer.on_notify(&event, self);
        }

        // Place the observers back into the game state.
        self.observers = observers;
    }

    pub fn tick(&mut self) -> Result<()> {
        // WARNING hack!!! sending a direction when it is not needed
        self.update_zombie(Direction::Up); // Check for collisions or any other periodic logic

        // render the updated game state
        let mut stdout = io::stdout();
        render_screen(&mut stdout, &self.zombies, &self.hero, self.screen_size)?;

        // check for collisions
        if self.detect_zombie_collision_hero() {
            self.notify_observers(GameEvent::HeroKilled);
        }

        Ok(())
    }

    // The `any` method tests whether any element of the iterator matches a predicate and returns `true` as soon as it finds a match
    pub fn detect_zombie_collision_hero(&self) -> bool {
        self.zombies
            .iter()
            .any(|zombie| zombie.position == self.hero.position)
    }

    // adds specified number of zombies to random positions
    pub fn add_zombies(&mut self, no: i32) {
        for _counter in 0..no {
            self.zombies
                .push(Zombie::new(random_position_around_point(self.screen_size)));
        }
    }

    pub fn update_hero(&mut self, key: Direction) {
        self.hero.update(key, self.screen_size);
    }

    pub fn update_zombie(&mut self, key: Direction) {
        for zombie in &mut self.zombies {
            zombie.update(key, self.screen_size);
        }
    }
}
