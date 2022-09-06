use crate::{world::Entity, Point2d};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Zombie {
    pub position: Point2d,
    image: char,
}

impl Entity for Zombie {
    // associated (static) function, used as constructor
    fn new(position: Point2d, image: char) -> Self {
        let random_position = Point2d {
            x: thread_rng().gen_range(0..position.x),
            y: thread_rng().gen_range(0..position.y),
        };
        Self {
            position: random_position,
            image,
        }
    }
    fn update(&mut self, key: &str) {
        // ERROR unuses str to match trait, how fix?
        println!("I am updating")
    }
}
