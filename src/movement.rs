use crate::types::Point2d;

pub fn can_move_up(position: Point2d) -> bool {
    position.x > 0
}

pub fn can_move_down(position: Point2d, screen_size: Point2d) -> bool {
    position.x < screen_size.x - 1
}

pub fn can_move_left(position: Point2d) -> bool {
    position.y > 0
}

pub fn can_move_right(position: Point2d, screen_size: Point2d) -> bool {
    position.y < screen_size.y - 1
}

pub fn move_up(position: Point2d) -> Point2d {
    if can_move_up(position) {
        Point2d {
            x: position.x.saturating_sub(1),
            y: position.y,
        }
    } else {
        position
    }
}

pub fn move_down(position: Point2d, screen_size: Point2d) -> Point2d {
    if can_move_down(position, screen_size) {
        Point2d {
            x: position.x.saturating_add(1),
            y: position.y,
        }
    } else {
        position
    }
}

pub fn move_right(position: Point2d, screen_size: Point2d) -> Point2d {
    if can_move_right(position, screen_size) {
        Point2d {
            x: position.x,
            y: position.y.saturating_add(1),
        }
    } else {
        position
    }
}

pub fn move_left(position: Point2d) -> Point2d {
    if can_move_left(position) {
        Point2d {
            x: position.x,
            y: position.y.saturating_sub(1),
        }
    } else {
        position
    }
}
