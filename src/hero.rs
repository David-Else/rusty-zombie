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

    // fn update(&mut self, key: &str) {
    fn update(&mut self, key: HeroMove) {
        self.last_key = key;
        match key {
            // this method needs to know width and height to prevent it going out of range
            HeroMove::Up => {
                if self.position.x > 0 {
                    self.position.x -= 1
                }
            }
            HeroMove::Down => {
                if self.position.x < self.screen_size.x - 1 {
                    self.position.x += 1
                }
            }
            HeroMove::Right => {
                if self.position.y < self.screen_size.y - 1 {
                    self.position.y += 1
                }
            }
            HeroMove::Left => {
                if self.position.y > 0 {
                    self.position.y -= 1
                }
            }
            _ => {
                println!("Not compatible key!");
            }
        }
    }
}
