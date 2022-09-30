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
    pub position: Point2d,
    pub last_key: HeroMove,
}

impl Entity for Hero {
    fn new(position: Point2d) -> Self {
        Self {
            position,
            last_key: HeroMove::Nokey,
        }
    }

    // fn update(&mut self, key: &str) {
    fn update(&mut self, key: HeroMove) {
        self.last_key = key;
        match key {
            // this method needs to know width and height to prevent it going out of range
            HeroMove::Up => self.position.x -= 1,
            HeroMove::Down => self.position.x += 1,
            HeroMove::Right => self.position.y += 1,
            HeroMove::Left => self.position.y -= 1,
            _ => {
                println!("Not compatible key!");
            }
        }
    }
}
