use crate::{world::Entity, Direction, Point2d};

#[derive(Debug)]
pub struct Hero {
    pub screen_size: Point2d,
    pub position: Point2d,
    // pub last_key: Direction,
}

impl Entity for Hero {
    fn new(screen_size: Point2d, position: Point2d) -> Self {
        Self {
            screen_size,
            position,
            // last_key: Direction::Nokey,
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
    fn update(&mut self, key: Direction) {
        // self.last_key = key;
        match key {
            // this method needs to know width and height to prevent it going out of range
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Right => self.move_right(),
            Direction::Left => self.move_left(),
            _ => {
                println!("Not compatible key!");
            }
        }
    }
}
