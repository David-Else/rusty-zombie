use crate::{
    movement::{move_down, move_left, move_right, move_up},
    types::{Direction, Entity, Point2d},
};

#[derive(Debug)]
pub struct Hero {
    pub position: Point2d,
    pub direction: Direction,
}

impl Entity for Hero {
    fn new(position: Point2d) -> Self {
        Self {
            position,
            direction: Direction::Up,
        }
    }

    fn update(&mut self, direction: Direction, screen_size: Point2d) {
        self.direction = direction;
        match direction {
            Direction::Up => self.position = move_up(self.position),
            Direction::Down => self.position = move_down(self.position, screen_size),
            Direction::Right => self.position = move_right(self.position, screen_size),
            Direction::Left => self.position = move_left(self.position),
            _ => println!("Hero cannot move in this direction!"),
        }
    }
}
