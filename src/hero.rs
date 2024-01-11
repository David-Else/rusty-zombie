use crate::{
    movement::{move_down, move_left, move_right, move_up},
    types::{Direction, Entity, Movable, Point2d},
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
            // default position needed so bullet can be fired before movement
            direction: Direction::Up,
        }
    }

    fn update(&mut self, screen_size: Point2d) {
        self.update_position(screen_size);
    }
}

impl Movable for Hero {
    fn update_position(&mut self, screen_size: Point2d) {
        self.position = match self.direction {
            Direction::Up => move_up(self.position),
            Direction::Down => move_down(self.position, screen_size),
            Direction::Right => move_right(self.position, screen_size),
            Direction::Left => move_left(self.position),
            // No default case needed since we are covering all variants of the Direction enum
        };
    }
}
