use crate::Point2d;

pub struct Hero {
    pub position: Point2d,
}

impl Hero {
    pub fn new(width: usize, height: usize) -> Self {
        let position = Point2d {
            x: width / 2,
            y: height / 2,
        };
        Self { position }
    }

    pub fn keys(&mut self, key: String) {
        match key.as_str() {
            "k" => {
                println!("Up");
                self.position.x += 1
            }
            "j" => {
                println!("Down");
                self.position.x -= 1
            }
            "l" => {
                println!("Right");
                self.position.y += 1
            }
            "h" => {
                println!("Left");
                self.position.y -= 1
            }
            _ => {
                println!("Not compatible key!");
            }
        }
        self.print_position()
    }

    fn print_position(&self) {
        println!(
            "The position is: x: {:?}, y:{:?}",
            self.position.x, self.position.y
        )
    }
}
