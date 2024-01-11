use crate::{
    movement::{move_down, move_left, move_right, move_up},
    types::{Direction, Point2d},
};

#[derive(Debug)]
pub struct Bullet {
    pub position: Point2d,
    move_every_n_ticks: usize,
    pub direction: Direction,
    tick_counter: usize,
}

impl Bullet {
    pub fn new(position: Point2d, direction: Direction) -> Self {
        Self {
            position,
            move_every_n_ticks: 25,
            direction,
            tick_counter: 0,
        }
    }

    pub fn update(&mut self, screen_size: Point2d) {
        self.tick_counter += 1;
        if self.tick_counter >= self.move_every_n_ticks {
            match self.direction {
                Direction::Up => self.position = move_up(self.position),
                Direction::Down => self.position = move_down(self.position, screen_size),
                Direction::Right => self.position = move_right(self.position, screen_size),
                Direction::Left => self.position = move_left(self.position),
                _ => println!("Hero cannot move in this direction!"),
            };
            self.tick_counter = 0; // Reset the counter after the move
        }
    }
}
