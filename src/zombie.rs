use rand::Rng;

use crate::{world::Entity, Direction, Point2d};

#[derive(Debug)]
pub struct Zombie {
    pub screen_size: Point2d,
    pub position: Point2d,
    move_every_n_ticks: usize,
    tick_counter: usize,
}

impl Entity for Zombie {
    // associated (static) function, used as constructor
    fn new(screen_size: Point2d, random_position: Point2d) -> Self {
        Self {
            screen_size,
            position: random_position,
            move_every_n_ticks: 50,
            tick_counter: 0,
        }
    }
    fn update(&mut self, _key: Direction) {
        // Generate a random number from 0 to 3
        let mut rng = rand::thread_rng();
        let dir = rng.gen_range(0..4);

        // Increment the tick counter
        self.tick_counter += 1;

        // Check if it's time to move
        if self.tick_counter >= self.move_every_n_ticks {
            match dir {
                0 => self.move_up(),
                1 => self.move_down(),
                2 => self.move_right(),
                3 => self.move_left(),
                _ => unreachable!(), // We only generate numbers 0-3, so this case is impossible
            }
            // Reset the counter
            self.tick_counter = 0;
        }

        // Match the generated number to a direction and move the zombie
    }

    fn move_up(&mut self) {
        if self.position.x > 0 {
            self.position.x -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.position.x < self.screen_size.x - 1 {
            self.position.x += 1;
        }
    }

    fn move_right(&mut self) {
        if self.position.y < self.screen_size.y - 1 {
            self.position.y += 1;
        }
    }

    fn move_left(&mut self) {
        if self.position.y > 0 {
            self.position.y -= 1;
        }
    }
}
