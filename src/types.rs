#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Entity {
    fn update(&mut self, screen_size: Point2d);
    fn new(position: Point2d) -> Self;
}

pub trait Movable {
    fn update_position(&mut self, screen_size: Point2d);
}
