use crate::{hero::Hero, zombie::Zombie, Point2d};
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
        let zombies: Vec<Zombie> = vec![];
        let heroes: Vec<Hero> = vec![];
        // create 2d array (matrix) as vector to represent the screen
        let screen = vec![vec![' '; screen_size.x]; screen_size.y];
        Self {
            zombies,
            heroes,
            screen,
        }
    }
    pub fn add_hero(&mut self, image: char) {
        let hero = Hero::new(
            Point2d {
                x: self.screen.len() / 2,
                y: self.screen[0].len() / 2,
            },
            image,
        );
        self.heroes.push(hero);
    }
    pub fn add_zombie(&mut self, image: char) {
        let zombie = Zombie::new(Point2d { x: 16, y: 16 }, image);
        self.zombies.push(zombie);
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
        print!("\x1B[2J\x1B[1;1H"); // clear screen
        for zombie in self.zombies.iter() {
            self.screen[zombie.position.x][zombie.position.y] = 'z';
        }
        for hero in self.heroes.iter() {
            self.screen[hero.position.x][hero.position.y] = 'h';
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
