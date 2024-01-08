use crate::{hero::Hero, random::random_position_around_point, zombie::Zombie, Direction, Point2d};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal::{self, size},
    ExecutableCommand, QueueableCommand,
};
use std::io::{Result, Stdout, Write};

pub trait Entity {
    fn update(&mut self, direction: Direction, screen_size: Point2d);
    fn new(position: Point2d) -> Self;
}

#[derive(Debug)]
pub struct GameState {
    pub zombies: Vec<Zombie>,
    pub hero: Hero,
    screen_size: Point2d,
}

impl GameState {
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

    fn is_collision(&self, point1: Point2d, point2: Point2d) -> bool {
        point1.x == point2.x && point1.y == point2.y
    }

    pub fn print_middle_screen(stdout: &mut Stdout, text: &str) -> Result<()> {
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

    pub fn render_screen(&mut self, mut stdout: &Stdout) -> Result<()> {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        // stdout.queue(style::Print(format!("{:?}", self.heroes[0])))?;

        for y in 0..self.screen_size.y {
            for x in 0..self.screen_size.x {
                if (y == 0 || y == self.screen_size.y - 1)
                    || (x == 0 || x == self.screen_size.x - 1)
                {
                    stdout
                        .queue(cursor::MoveTo(y as u16, x as u16))?
                        .queue(style::PrintStyledContent("█".grey()))?;
                }
            }
        }

        // zombies
        for zombie in self.zombies.iter() {
            stdout
                .queue(cursor::MoveTo(
                    zombie.position.y as u16,
                    zombie.position.x as u16,
                ))?
                .queue(style::PrintStyledContent("z".green()))?;
        }

        // hero
        stdout
            .queue(cursor::MoveTo(
                self.hero.position.y as u16,
                self.hero.position.x as u16,
            ))?
            .queue(style::PrintStyledContent("h".red()))?;

        // draw screen from queued buffer
        stdout.flush()?;
        Ok(())
    }
}
