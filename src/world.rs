use crate::{hero::Hero, zombie::Zombie, Point2d};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;
use std::io::{Stdout, Write};
#[derive(Debug)]

pub struct GameState {
    zombies: Vec<Zombie>,
    heroes: Vec<Hero>,
    screen_size: Point2d,
}

pub trait Entity {
    fn update(&mut self, key: &str);
    fn new(position: Point2d) -> Self;
}

impl GameState {
    pub fn new(screen_size: Point2d) -> Self {
        let zombies: Vec<Zombie> = vec![];
        let heroes: Vec<Hero> = vec![];
        Self {
            zombies,
            heroes,
            screen_size,
        }
    }

    // adds hero to the middle of the screen
    pub fn add_hero(&mut self) {
        self.heroes.push(Hero::new(Point2d {
            x: self.screen_size.x / 2,
            y: self.screen_size.y / 2,
        }));
    }

    fn calculate_random_position_around_point(&mut self, mid_point: Point2d) -> Point2d {
        let minimum_r = self.screen_size.y / 2;

        let rn: f64 = thread_rng().gen(); //.gen_range(0..1);
        let theta = rn * (2.0 * PI) as f64;

        let r: f64 = (thread_rng()
            .gen_range((((minimum_r as f64) / 2.0).floor()) as usize..minimum_r))
            as f64; // * (variation_in_r + minimum_r) as f64;

        Point2d {
            x: (((theta.cos() * r).floor() as isize) + (mid_point.x) as isize) as usize,
            y: (((theta.sin() * r).floor() as isize) + (mid_point.y) as isize) as usize,
        }
    }

    // adds specified number of zombies to random positions
    pub fn add_zombies(&mut self, no: i32) {
        for _counter in 0..no {
            // create random position on each itteration and wipe it next time
            // use middle of screen as hero position, only to avoid borrow error using actual position :)
            let random_pos = self.calculate_random_position_around_point(Point2d {
                x: self.screen_size.x / 2,
                y: self.screen_size.y / 2,
            });
            self.zombies.push(Zombie::new(random_pos));
        }
    }

    pub fn update(&mut self, key: &str) {
        // https://www.reddit.com/r/learnrust/comments/x76d3o/how_do_i_iterate_over_a_vector_with_a_for_in_loop/
        for hero in &mut self.heroes {
            hero.update(key);
        }
    }

    pub fn render_screen(&mut self, mut stdout: &Stdout) -> Result<()> {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        // border
        for y in 0..self.screen_size.y {
            for x in 0..self.screen_size.x {
                if (y == 0 || y == self.screen_size.y - 1)
                    || (x == 0 || x == self.screen_size.x - 1)
                {
                    stdout
                        .queue(cursor::MoveTo(x as u16, y as u16))?
                        .queue(style::PrintStyledContent("█".grey()))?;
                }
            }
        }
        // zombies
        for zombie in self.zombies.iter() {
            stdout
                .queue(cursor::MoveTo(
                    zombie.position.x as u16,
                    zombie.position.y as u16,
                ))?
                .queue(style::PrintStyledContent("z".green()))?;
        }
        // hero
        for hero in self.heroes.iter() {
            stdout
                .queue(cursor::MoveTo(
                    hero.position.x as u16,
                    hero.position.y as u16,
                ))?
                .queue(style::PrintStyledContent("h".red()))?;
        }
        // draw screen from queued buffer
        stdout.flush()?;
        Ok(())
    }
}
