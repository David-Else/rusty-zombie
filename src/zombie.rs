use crate::Point2d;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Zombie {
    pub position: Point2d,
    image: char,
}

impl Zombie {
    // associated (static) function, used as constructor
    pub fn new(max_width: usize, max_height: usize, image: char) -> Self {
        let position = Point2d {
            x: thread_rng().gen_range(0..max_width),
            y: thread_rng().gen_range(0..max_height),
        };
        Self { position, image }
    }
    pub fn update(&self) {
        println!("I am updating")
    }
}
