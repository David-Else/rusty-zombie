use crate::{
    movement::{move_down, move_left, move_right, move_up},
    types::{Direction, Entity, Movable, Point2d},
};

#[derive(Debug)]
pub struct Hero {
    pub position: Point2d,
    pub direction: Direction,
    pub lives: i32,
}

impl Hero {
    pub fn move_in_direction(&mut self, direction: Direction, screen_size: Point2d) {
        self.direction = direction;
        self.update_position(screen_size);
    }
}

impl Entity for Hero {
    fn new(position: Point2d) -> Self {
        Self {
            position,
            // default position needed so bullet can be fired before movement
            direction: Direction::Up,
            lives: 3,
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
