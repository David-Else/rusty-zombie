use crate::{
    movement::{move_down, move_left, move_right, move_up},
    world::Entity,
    Direction, Point2d,
};

#[derive(Debug)]
pub struct Hero {
    pub position: Point2d,
}

impl Entity for Hero {
    fn new(position: Point2d) -> Self {
        Self { position }
    }

    fn update(&mut self, direction: Direction, screen_size: Point2d) {
        match direction {
            Direction::Up => self.position = move_up(self.position),
            Direction::Down => self.position = move_down(self.position, screen_size),
            Direction::Right => self.position = move_right(self.position, screen_size),
            Direction::Left => self.position = move_left(self.position),
            _ => println!("Hero cannot move in this direction!"),
        }
    }
}
