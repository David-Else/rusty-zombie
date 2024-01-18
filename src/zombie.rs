use crate::{
    movement::{move_down, move_left, move_right, move_up},
    random::{random_direction, random_u16_in_inclusive_range},
    types::{Direction, Entity, Movable, Point2d},
};

#[derive(Debug)]
pub struct Zombie {
    pub position: Point2d,
    ticks_between_moves: usize,
    tick_counter: usize,
}

impl Entity for Zombie {
    fn new(position: Point2d) -> Self {
        Self {
            position,
            ticks_between_moves: random_u16_in_inclusive_range(40, 50) as usize,
            tick_counter: 0,
        }
    }

    fn update(&mut self, screen_size: Point2d) {
        self.update_position(screen_size);
    }
}

impl Movable for Zombie {
    fn update_position(&mut self, screen_size: Point2d) {
        self.tick_counter += 1;
        if self.tick_counter >= self.ticks_between_moves {
            // Generate a random direction for the zombie to move in
            // TODO return actual direction not a number
            let direction = match random_direction() {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Right,
                3 => Direction::Left,
                _ => unreachable!(),
            };

            // Move the zombie in the generated random direction
            self.position = match direction {
                Direction::Up => move_up(self.position),
                Direction::Down => move_down(self.position, screen_size),
                Direction::Right => move_right(self.position, screen_size),
                Direction::Left => move_left(self.position),
            };

            self.tick_counter = 0;
        }
    }
}
