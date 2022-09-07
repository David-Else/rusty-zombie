use crate::{hero::Hero, zombie::Zombie, Point2d};
#[derive(Debug)]

pub struct GameState {
    zombies: Vec<Zombie>,
    heroes: Vec<Hero>,
    screen: Vec<Vec<char>>,
}

pub trait Entity {
    fn update(&mut self, key: &str);
    fn new(position: Point2d<u8>, image: char) -> Self;
}

impl GameState {
    pub fn new(screen_size: &Point2d<u8>) -> Self {
        let zombies: Vec<Zombie> = vec![];
        let heroes: Vec<Hero> = vec![];
        // create 2d array (matrix) as vector to represent the screen
        let screen = vec![vec![' '; screen_size.x as usize]; screen_size.y as usize];
        Self {
            zombies,
            heroes,
            screen,
        }
    }
    // get array size in usize for indexing
    // pub fn screen_dimensions(&self) {
    //     return Point2d {
    //         x: self.screen.len() as usize,
    //         y: self.screen[0].len() as usize,
    //     };
    // }
    // adds hero to the middle of the screen
    pub fn add_hero(&mut self, image: char) {
        self.heroes.push(Hero::new(
            Point2d {
                x: self.screen.len() as u8 / 2,
                y: self.screen[0].len() as u8 / 2,
            },
            image,
        ));
    }
    // adds specified number of zombies to random positions
    pub fn add_zombies(&mut self, no: i32, image: char) {
        for _counter in 0..no {
            self.zombies.push(Zombie::new(
                Point2d {
                    x: self.screen.len() as u8,
                    y: self.screen[0].len() as u8,
                },
                image,
            ));
        }
    }
    pub fn update(&mut self, key: &str) {
        // for zombie in self.zombies.iter() {
        //     zombie.update();
        // }
        for hero in self.heroes.iter_mut() {
            hero.update(key);
        }
    }
    pub fn render_screen(&mut self) {
        // print!("\x1B[2J\x1B[1;1H"); // clear screen
        for zombie in self.zombies.iter() {
            self.screen[zombie.position.x as usize][zombie.position.y as usize] = 'z';
        }
        for hero in self.heroes.iter() {
            self.screen[hero.position.x as usize][hero.position.y as usize] = 'h';
        }
        for linevector in self.screen.iter() {
            // other way to do it, debug means no blank spaces
            // println!("{:?}", linevector)
            for column in linevector.iter() {
                print!("{}", column)
            }
            println!();
        }
    }
}
