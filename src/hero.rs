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
        self.move_in_direction(Some(self.direction), screen_size);
    }
}

impl Movable for Hero {
    fn move_in_direction(&mut self, direction: Option<Direction>, screen_size: Point2d) {
        // For `Hero`, `direction` should always be `Some`.
        if let Some(dir) = direction {
            match dir {
                Direction::Up => self.position = move_up(self.position),
                Direction::Down => self.position = move_down(self.position, screen_size),
                Direction::Right => self.position = move_right(self.position, screen_size),
                Direction::Left => self.position = move_left(self.position),
            }
        } else {
            // Handle the None case or assume it will never happen.
            unreachable!("Hero requires a direction to move.");
        }
    }
}
