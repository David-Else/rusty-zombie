use crate::Point2d;
use rand::{thread_rng, Rng};

pub struct Zombie {
    pub position: Point2d,
}

impl Zombie {
    // associated (static) function, used as constructor
    pub fn new(width: usize, height: usize) -> Self {
        let position = Point2d {
            x: thread_rng().gen_range(0..width),
            y: thread_rng().gen_range(0..height),
        };
        Self { position }
    }
}
