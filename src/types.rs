#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Entity {
    fn update(&mut self, direction: Direction, screen_size: Point2d);
    fn new(position: Point2d) -> Self;
}
