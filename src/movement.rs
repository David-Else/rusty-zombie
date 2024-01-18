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

// #[cfg(test)]
// #[test]
// fn test_movement_within_bounds() {
//     let screen_size = Point2d { x: 100, y: 100 };
//     let mut position = Point2d { x: 50, y: 50 };

//     assert!(can_move_up(position));
//     position = move_up(position);
//     assert_eq!(position.x, 49);

//     assert!(can_move_down(position, screen_size));
//     position = move_down(position, screen_size);
//     assert_eq!(position.x, 50);

//     assert!(can_move_left(position));
//     position = move_left(position);
//     assert_eq!(position.y, 49);

//     assert!(can_move_right(position, screen_size));
//     position = move_right(position, screen_size);
//     assert_eq!(position.y, 50);
// }

// #[test]
// fn test_movement_at_boundaries() {
//     let screen_size = Point2d { x: 100, y: 100 };
//     let mut position = Point2d { x: 0, y: 0 };

//     assert!(!can_move_up(position));
//     position = move_up(position);
//     assert_eq!(position.x, 0); // x should not change because it's at the top boundary

//     assert!(can_move_down(position, screen_size));
//     position = move_down(position, screen_size);
//     assert_eq!(position.x, 1);

//     assert!(!can_move_left(position));
//     position = move_left(position);
//     assert_eq!(position.y, 0); // y should not change because it's at the left boundary

//     assert!(can_move_right(position, screen_size));
//     position = move_right(position, screen_size);
//     assert_eq!(position.y, 1);
// }

// #[test]
// fn test_movement_at_max_value() {
//     let screen_size = Point2d {
//         x: u16::MAX,
//         y: u16::MAX,
//     };
//     let mut position = Point2d {
//         x: u16::MAX - 1,
//         y: u16::MAX - 1,
//     };

//     // Test moving down and right at the edge of u16 max value
//     assert!(can_move_down(position, screen_size));
//     position = move_down(position, screen_size);
//     assert_eq!(position.x, u16::MAX);

//     assert!(can_move_right(position, screen_size));
//     position = move_right(position, screen_size);
//     assert_eq!(position.y, u16::MAX);

//     // At the maximum edge, trying to move further should keep the position the same
//     assert!(!can_move_down(position, screen_size));
//     position = move_down(position, screen_size);
//     assert_eq!(position.x, u16::MAX); // Should not change from the max value

//     assert!(!can_move_right(position, screen_size));
//     position = move_right(position, screen_size);
//     assert_eq!(position.y, u16::MAX); // Should not change from the max value
// }
// #[test]
// fn test_random_position_within_screen_bounds() {
//     let screen_size = Point2d { x: 200, y: 300 };
//     for _ in 0..1000 {
//         let position = random_position_around_point(screen_size);
//         // Ensure that the position is within bounds
//         assert!(
//             position.x < screen_size.x && position.y < screen_size.y,
//             "random_position_around_point() returned a position out of bounds: {:?}",
//             position
//         );
//     }
// }
