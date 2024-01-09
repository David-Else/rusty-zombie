use crate::{
    movement::{move_down, move_left, move_right, move_up},
    random::{random_direction, random_usize_in_inclusive_range},
    types::{Direction, Entity, Point2d},
};

#[derive(Debug)]
pub struct Zombie {
    pub position: Point2d,
    move_every_n_ticks: usize,
    tick_counter: usize,
}

impl Entity for Zombie {
    // associated (static) function, used as constructor
    fn new(position: Point2d) -> Self {
        Self {
            position,
            move_every_n_ticks: random_usize_in_inclusive_range(40, 50),
            tick_counter: 0,
        }
    }

    fn update(&mut self, _direction: Direction, screen_size: Point2d) {
        self.tick_counter += 1;
        if self.tick_counter >= self.move_every_n_ticks {
            self.position = match random_direction() {
                0 => move_up(self.position),
                1 => move_down(self.position, screen_size),
                2 => move_right(self.position, screen_size),
                3 => move_left(self.position),
                _ => unreachable!(), // We only generate numbers 0-3, so this should never happen
            };
            self.tick_counter = 0; // Reset the counter after the move
        }
    }
}
