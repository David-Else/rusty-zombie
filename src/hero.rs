use crate::{world::Entity, Point2d};

#[derive(Debug, Clone, Copy)]
pub enum HeroMove {
    Up,
    Down,
    Left,
    Right,
    Nokey,
}

#[derive(Debug)]
pub struct Hero {
    pub screen_size: Point2d,
    pub position: Point2d,
    pub last_key: HeroMove,
}

impl Entity for Hero {
    fn new(screen_size: Point2d, position: Point2d) -> Self {
        Self {
            screen_size,
            position,
            last_key: HeroMove::Nokey,
        }
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

    // fn update(&mut self, key: &str) {
    fn update(&mut self, key: HeroMove) {
        self.last_key = key;
        match key {
            // this method needs to know width and height to prevent it going out of range
            HeroMove::Up => self.move_up(),
            HeroMove::Down => self.move_down(),
            HeroMove::Right => self.move_right(),
            HeroMove::Left => self.move_left(),
            _ => {
                println!("Not compatible key!");
            }
        }
    }
}
