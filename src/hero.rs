use crate::{world::Entity, Point2d};

#[derive(Debug)]
pub struct Hero {
    pub position: Point2d,
}

impl Entity for Hero {
    fn new(position: Point2d) -> Self {
        Self { position }
    }

    fn update(&mut self, key: &str) {
        match key {
            // this method needs to know width and height to prevent it going out of range
            "k" => self.position.y -= 1,
            "j" => self.position.y += 1,
            "l" => self.position.x += 1,
            "h" => self.position.x -= 1,
            _ => {
                println!("Not compatible key!");
            }
        }
    }
}
