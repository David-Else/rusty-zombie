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
            direction: Direction::Up,
        }
    }

    fn update(&mut self, screen_size: Point2d) {
        self.move_in_direction(screen_size); // Now it doesn't take a direction parameter
    }
}

impl Movable for Hero {
    fn move_in_direction(&mut self, screen_size: Point2d) {
        self.position = match self.direction {
            Direction::Up => move_up(self.position),
            Direction::Down => move_down(self.position, screen_size),
            Direction::Right => move_right(self.position, screen_size),
            Direction::Left => move_left(self.position),
            // No default case needed since we are covering all variants of the Direction enum
        };
    }
}
