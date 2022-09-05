use crate::Point2d;

#[derive(Debug)]
pub struct Hero {
    pub position: Point2d,
    image: char,
}

impl Hero {
    pub fn new(x: usize, y: usize, image: char) -> Self {
        let position = Point2d { x, y };
        Self { position, image }
    }

    pub fn update(&mut self, key: &str) {
        match key {
            // goes out of range and crashes
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
