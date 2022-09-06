use crate::{hero::Hero, zombie::Zombie};
#[derive(Debug)]

pub struct GameState {
    zombies: Vec<Zombie>,
    heroes: Vec<Hero>,
    screen: Vec<Vec<char>>,
}

impl GameState {
    pub fn new() -> Self {
        let zombies: Vec<Zombie> = vec![];
        let heroes: Vec<Hero> = vec![];
        // create 2d array (matrix) as vector to represent the screen
        let screen = vec![vec![' '; 16]; 16];
        Self {
            zombies,
            heroes,
            screen,
        }
    }
    pub fn add_hero(&mut self, x: usize, y: usize, image: char) {
        let hero = Hero::new(x, y, image);
        self.heroes.push(hero);
    }
    pub fn add_zombie(&mut self, image: char) {
        let zombie = Zombie::new(16, 16, image);
        self.zombies.push(zombie);
    }
    pub fn update(&mut self, key: &str) {
        for zombie in self.zombies.iter() {
            zombie.update();
        }
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
