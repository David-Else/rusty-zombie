use crate::{world::Entity, Point2d};

#[derive(Debug)]
pub struct Hero {
    pub position: Point2d,
    image: char,
}

impl Entity for Hero {
    fn new(position: Point2d, image: char) -> Self {
        Self { position, image }
    }

    fn update(&mut self, key: &str) {
        match key {
            // this method needs to know width and height to prevent it going out of range
            "k" => self.position.x += 1,
            "j" => self.position.x -= 1,
            "l" => self.position.y += 1,
            "h" => self.position.y -= 1,
            _ => {
                println!("Not compatible key!");
            }
        }
    }
}
