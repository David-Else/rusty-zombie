use crate::{hero::HeroMove, world::Entity, Point2d};

#[derive(Debug)]
pub struct Zombie {
    pub screen_size: Point2d,
    pub position: Point2d,
}

impl Entity for Zombie {
    // associated (static) function, used as constructor
    fn new(screen_size: Point2d, random_position: Point2d) -> Self {
        Self {
            screen_size,
            position: random_position,
        }
    }
    fn update(&mut self, _key: HeroMove) {
        // ERROR unuses str to match trait, how fix?
        println!("I am updating")
    }

    fn move_up(&mut self) {
        todo!()
    }

    fn move_down(&mut self) {
        todo!()
    }

    fn move_left(&mut self) {
        todo!()
    }

    fn move_right(&mut self) {
        todo!()
    }
}
