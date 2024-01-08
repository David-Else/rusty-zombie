use crate::{
    hero::Hero, random::random_position_around_point, render::render_screen, zombie::Zombie,
    Direction, Point2d,
};
use crossterm::{cursor, terminal::size, QueueableCommand};
use std::io::{self, Result, Stdout, Write};

pub trait Entity {
    fn update(&mut self, direction: Direction, screen_size: Point2d);
    fn new(position: Point2d) -> Self;
}

#[derive(Debug)]
pub struct GameState {
    // fields representing the state of the game
    pub zombies: Vec<Zombie>,
    pub hero: Hero,
    screen_size: Point2d,
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
            self.print_middle_screen(&mut stdout, "You are dead!")?;
        }

        Ok(())
    }

    fn is_collision(&self, point1: Point2d, point2: Point2d) -> bool {
        point1.x == point2.x && point1.y == point2.y
    }

    fn print_middle_screen(&self, stdout: &mut Stdout, text: &str) -> Result<()> {
        let (cols, rows) = size()?; // Get the number of columns and rows of the terminal window

        // Move cursor to the calculated position and print the text
        stdout
            .queue(cursor::MoveTo(cols / 2, rows / 2))?
            .queue(crossterm::style::Print(text))? // Print the text
            .flush()?; // Flush the stdout to immediately output the text

        Ok(())
    }

    pub fn detect_zombie_collision_hero(&mut self) -> bool {
        for zombie in self.zombies.iter() {
            if self.is_collision(
                Point2d {
                    x: zombie.position.x,
                    y: zombie.position.y,
                },
                Point2d {
                    x: self.hero.position.x,
                    y: self.hero.position.y,
                },
            ) {
                return true;
            }
        }

        false
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
