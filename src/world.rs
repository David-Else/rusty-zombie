use crate::{
    bullets::Bullet,
    events::{GameEvent, Observer},
    hero::Hero,
    random::random_position_around_point,
    types::{Entity, Point2d},
    zombie::Zombie,
};

pub enum Screen {
    StartMenu,
    GamePlay,
    GameOver,
}

// Methods to manage the game state
pub trait GameLogic {
    fn update_state(&mut self);
    fn check_collisions(&mut self);
    fn register_observer(&mut self, observer: Box<dyn Observer>);
    fn notify_observers(&mut self, event: GameEvent);
}

// #[derive(Debug)]
pub struct GameState {
    pub zombies: Vec<Zombie>,
    pub bullets: Vec<Bullet>,
    pub hero: Hero,
    pub screen_size: Point2d,
    pub current_screen: Screen,
    pub is_running: bool,
    observers: Vec<Box<dyn Observer>>,
}

impl GameState {
    pub fn new(screen_size: Point2d) -> Self {
        Self {
            zombies: Vec::new(), // The compiler knows this holds elements of type `Zombie` variable
            bullets: Vec::new(),
            hero: Hero::new(Point2d {
                x: screen_size.x / 2,
                y: screen_size.y / 2,
            }),
            screen_size,
            current_screen: Screen::StartMenu,
            is_running: true,
            observers: vec![],
        }
    }

    pub fn add_zombies(&mut self, no: i32) {
        for _counter in 0..no {
            self.zombies
                .push(Zombie::new(random_position_around_point(self.screen_size)));
        }
    }

    pub fn add_bullet(&mut self) {
        self.bullets
            .push(Bullet::new(self.hero.position, self.hero.direction));
    }
}

impl GameLogic for GameState {
    fn check_collisions(&mut self) {
        // Check if the hero collides with any zombies
        if self
            .zombies
            .iter()
            .any(|zombie| zombie.position == self.hero.position)
        {
            self.notify_observers(GameEvent::HeroKilled);
        }
    }

    fn register_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    // Rust's rules prevent you from calling a method with `&mut self` while iterating over a collection of references (`&self.observers`).
    // There's a rule that you cannot have multiple mutable references to the same data
    fn notify_observers(&mut self, event: GameEvent) {
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

    fn update_state(&mut self) {
        // Update bullets and zombies, but hero update is called directly from input handling
        for bullet in &mut self.bullets {
            bullet.update(self.screen_size);
        }
        for zombie in &mut self.zombies {
            zombie.update(self.screen_size);
        }
    }
}
