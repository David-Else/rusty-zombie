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
    pub fn add_zombies(&mut self, no: i32, image: char) {
        for _counter in 0..no {
            self.zombies.push(Zombie::new(
                Point2d {
                    x: self.screen.len(),
                    y: self.screen[0].len(),
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

    pub fn render_screen(&mut self) {
        // reset date on screen screen, best way to do it? repeating new = duplication
        self.screen = vec![vec![' '; self.screen_width()]; self.screen_height()];

        const BORDER_CHAR: char = '.';
        let border_width = self.screen[0].len() + 2;

        print!("\x1B[2J\x1B[1;1H"); // clear terminal screen
        for zombie in self.zombies.iter() {
            self.screen[zombie.position.x as usize][zombie.position.y as usize] = 'z';
        }
        for hero in self.heroes.iter() {
            self.screen[hero.position.x as usize][hero.position.y as usize] = 'h';
        }

        println!("{}", str::repeat(&BORDER_CHAR.to_string(), border_width));
        for linevector in self.screen.iter() {
            println!(
                "{BORDER_CHAR}{}{BORDER_CHAR}",
                linevector.iter().collect::<String>()
            );
        }
        println!("{}", str::repeat(&BORDER_CHAR.to_string(), border_width));

        // for linevector in self.screen.iter() {
        //     // other way to do it, debug means no blank spaces
        //     // println!("{:?}", linevector)
        //     for column in linevector.iter() {
        //         print!("{}", column)
        //     }
        //     println!();
        // }
    }
}
