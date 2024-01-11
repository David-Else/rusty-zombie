use crate::{
    movement::{move_down, move_left, move_right, move_up},
    random::{random_direction, random_usize_in_inclusive_range},
    types::{Direction, Entity, Movable, Point2d},
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

    fn update(&mut self, screen_size: Point2d) {
        self.move_in_direction(screen_size);
    }
}

impl Movable for Zombie {
    fn move_in_direction(&mut self, screen_size: Point2d) {
        self.tick_counter += 1;
        if self.tick_counter >= self.move_every_n_ticks {
            // Generate a random direction for the zombie to move in
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

            // Reset the counter after the zombie moves
            self.tick_counter = 0;
        }
    }
}
