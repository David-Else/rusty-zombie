use crate::types::Point2d;

// it is a good practice to use `saturating_add` when adding to `u16` fields of `Point2d` to prevent overflow. Similarly, `saturating_sub` should be used when subtracting to prevent underflow.

pub fn can_move_up(position: Point2d) -> bool {
    position.x > 0
}

pub fn can_move_down(position: Point2d, screen_size: Point2d) -> bool {
    position.x < screen_size.x.saturating_sub(1)
}

pub fn can_move_left(position: Point2d) -> bool {
    position.y > 0
}

pub fn can_move_right(position: Point2d, screen_size: Point2d) -> bool {
    position.y < screen_size.y.saturating_sub(1)
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

#[cfg(test)]
#[test]
fn test_movement_within_bounds() {
    let screen_size = Point2d { x: 100, y: 100 };
    let mut position = Point2d { x: 50, y: 50 };

    assert!(can_move_up(position));
    position = move_up(position);
    assert_eq!(position.x, 49);

    assert!(can_move_down(position, screen_size));
    position = move_down(position, screen_size);
    assert_eq!(position.x, 50);

    assert!(can_move_left(position));
    position = move_left(position);
    assert_eq!(position.y, 49);

    assert!(can_move_right(position, screen_size));
    position = move_right(position, screen_size);
    assert_eq!(position.y, 50);
}

#[test]
fn test_movement_at_boundaries() {
    let screen_size = Point2d { x: 100, y: 100 };
    let mut position = Point2d { x: 0, y: 0 };

    assert!(!can_move_up(position));
    position = move_up(position);
    assert_eq!(position.x, 0); // x should not change because it's at the top boundary

    assert!(can_move_down(position, screen_size));
    position = move_down(position, screen_size);
    assert_eq!(position.x, 1);

    assert!(!can_move_left(position));
    position = move_left(position);
    assert_eq!(position.y, 0); // y should not change because it's at the left boundary

    assert!(can_move_right(position, screen_size));
    position = move_right(position, screen_size);
    assert_eq!(position.y, 1);
}
