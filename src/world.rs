use crate::{
    bullets::Bullet,
    events::Observer,
    hero::Hero,
    input::GameInput,
    random::random_position_around_point,
    types::{Direction, Entity, Point2d},
    zombie::Zombie,
};

pub enum GameEvent {
    HeroKilled,
}

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

    pub fn handle_game_input(&mut self, input: GameInput) {
        match (&self.current_screen, input) {
            // start menu
            (Screen::StartMenu, GameInput::Start) => {
                self.current_screen = Screen::GamePlay;
            }
            (Screen::StartMenu, GameInput::Exit) => {
                self.is_running = false;
            }

            // gameplay
            (Screen::GamePlay, GameInput::MoveUp) => {
                self.hero.move_in_direction(Direction::Up, self.screen_size)
            }
            (Screen::GamePlay, GameInput::MoveDown) => self
                .hero
                .move_in_direction(Direction::Down, self.screen_size),
            (Screen::GamePlay, GameInput::MoveLeft) => self
                .hero
                .move_in_direction(Direction::Left, self.screen_size),
            (Screen::GamePlay, GameInput::MoveRight) => self
                .hero
                .move_in_direction(Direction::Right, self.screen_size),
            (Screen::GamePlay, GameInput::Fire) => {
                self.add_bullet();
            }

            // game over
            (Screen::GameOver, GameInput::Start) => {
                self.current_screen = Screen::GamePlay; // For restarting the game
            }
            // ... handle other game over inputs
            _ => {}
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
        // To check for bullet-zombie collisions and remove the collided entities, Rust provides several ways to do so while adhering to its strict borrowing rules. Doing this in a simple loop is challenging because modifying the collection (e.g., removing items) while iterating over it is not allowed.

        // A common pattern is to collect indices of items to remove and then remove them after the iteration is complete. However, we must be cautious to not invalidate indices when removing items multiple times from the same vector. A solution is to remove the items in reverse order of their indices.

        // Find all bullet-zombie collisions
        let mut bullets_to_remove = Vec::new();
        let mut zombies_to_remove = Vec::new();

        for (bullet_idx, bullet) in self.bullets.iter().enumerate() {
            for (zombie_idx, zombie) in self.zombies.iter().enumerate() {
                if bullet.position == zombie.position {
                    bullets_to_remove.push(bullet_idx);
                    zombies_to_remove.push(zombie_idx);
                    break;
                }
            }
        }

        // Remove bullets and zombies that collided. Start from the end to preserve indexing.
        // `swap_remove` for efficiency removes an element from the vector and replaces it with the last element, maintaining the length of the vector but potentially changing the order of items.
        for &bullet_idx in bullets_to_remove.iter().rev() {
            self.bullets.swap_remove(bullet_idx);
        }
        for &zombie_idx in zombies_to_remove.iter().rev() {
            self.zombies.swap_remove(zombie_idx);
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
