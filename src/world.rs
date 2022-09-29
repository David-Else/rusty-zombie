use crate::{hero::Hero, zombie::Zombie, Point2d};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::{stdout, Stdout, Write};
#[derive(Debug)]

pub struct GameState {
    zombies: Vec<Zombie>,
    heroes: Vec<Hero>,
    screen: Vec<Vec<char>>,
}

pub trait Entity {
    fn update(&mut self, key: &str);
    fn new(position: Point2d, image: char) -> Self;
}

impl GameState {
    pub fn new(screen_size: &Point2d) -> Self {
        // create 2d array (matrix) as vector to represent the screen
        let width = screen_size.x;
        let height = screen_size.y;

        let zombies: Vec<Zombie> = vec![];
        let heroes: Vec<Hero> = vec![];
        let screen = vec![vec![' '; width]; height];
        Self {
            zombies,
            heroes,
            screen,
        }
    }

    // adds hero to the middle of the screen
    pub fn add_hero(&mut self, image: char) {
        self.heroes.push(Hero::new(
            Point2d {
                x: self.screen.len() / 2,
                y: self.screen[0].len() / 2,
            },
            image,
        ));
    }

    // adds specified number of zombies to random positions
    pub fn add_zombies(&mut self, no: i32, image: char, number_cols: usize, number_rows: usize) {
        for _counter in 0..no {
            self.zombies.push(Zombie::new(
                Point2d {
                    x: number_cols,
                    y: number_rows,
                },
                image,
            ));
        }
    }

    pub fn update(&mut self, key: &str) {
        // https://www.reddit.com/r/learnrust/comments/x76d3o/how_do_i_iterate_over_a_vector_with_a_for_in_loop/
        for hero in &mut self.heroes {
            hero.update(key);
        }
    }

    pub fn screen_width(&mut self) -> usize {
        self.screen.len()
    }

    pub fn screen_height(&mut self) -> usize {
        self.screen[0].len()
    }

    pub fn render_screen(&mut self, mut stdout: &Stdout) -> Result<()> {
        // let mut stdout = stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        for y in 0..self.screen_height() {
            for x in 0..self.screen_width() {
                if (y == 0 || y == self.screen_height() - 1)
                    || (x == 0 || x == self.screen_width() - 1)
                {
                    stdout
                        .queue(cursor::MoveTo(x as u16, y as u16))?
                        .queue(style::PrintStyledContent("█".grey()))?;
                }
            }
        }

        for zombie in self.zombies.iter() {
            stdout
                .queue(cursor::MoveTo(
                    zombie.position.x as u16,
                    zombie.position.y as u16,
                ))?
                .queue(style::PrintStyledContent("z".green()))?;
        }

        for hero in self.heroes.iter() {
            stdout
                .queue(cursor::MoveTo(
                    hero.position.x as u16,
                    hero.position.y as u16,
                ))?
                .queue(style::PrintStyledContent("h".red()))?;
        }

        stdout.flush()?;
        Ok(())
    }
}
