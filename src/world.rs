use crate::{
    hero::Hero, random::random_position_around_point, render::render_screen, zombie::Zombie,
    Direction, Point2d,
};
use std::io::{self, Result};

pub trait Entity {
    fn update(&mut self, direction: Direction, screen_size: Point2d);
    fn new(position: Point2d) -> Self;
}

pub struct GameUI;

// This observer is stateless so it does not need to be mutable, the struct is empty
impl Observer for GameUI {
    fn on_notify(&self, event: &GameEvent) {
        match event {
            GameEvent::HeroKilled => {
                // Update UI to show player death (e.g., a game over screen)
                println!("Game Over!");
            } // Handle any other UI-related events...
        }
    }
}

enum GameEvent {
    HeroKilled,
    // Other game events...
}

pub trait Observer {
    fn on_notify(&self, event: &GameEvent);
}

// #[derive(Debug)]
pub struct GameState {
    // fields representing the state of the game
    pub zombies: Vec<Zombie>,
    pub hero: Hero,
    screen_size: Point2d,
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

    fn notify_observers(&self, event: GameEvent) {
        for observer in &self.observers {
            observer.on_notify(&event);
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        // WARNING hack!!! sending a direction when it is not needed
        self.update_zombie(Direction::Up); // Check for collisions or any other periodic logic

        // render the updated game state
        let mut stdout = io::stdout();
        render_screen(&mut stdout, &self.zombies, &self.hero, self.screen_size)?;
        // check for collisions
        if self.detect_zombie_collision_hero() {
            // Notify observers that the hero has been killed
            self.notify_observers(GameEvent::HeroKilled);
            // print_middle_screen(&mut stdout, "You are dead!")?;
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
