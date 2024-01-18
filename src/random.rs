use crate::types::{Direction, Point2d};
use rand::{thread_rng, Rng};

pub fn random_u16_in_inclusive_range(min: u16, max: u16) -> u16 {
    thread_rng().gen_range(min..=max)
}

pub fn random_direction() -> Direction {
    match random_u16_in_inclusive_range(0, 3) {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Right,
        3 => Direction::Left,
        _ => unreachable!(),
    }
}

pub fn random_position_around_point(screen_size: Point2d) -> Point2d {
    let mid_point = Point2d {
        x: screen_size.x / 2,
        y: screen_size.y / 2,
    };

    let minimum_r = (screen_size.x / 2) as f64;

    let rn: f64 = thread_rng().gen(); // Generates a float between 0.0 and 1.0.
    let theta = rn * 2.0 * std::f64::consts::PI; // Full circle in radians.

    // Generate a random radius within the screen bounds as a float.
    let r = thread_rng().gen_range(minimum_r / 2.0..=minimum_r);

    // Calculate the components offset by the random angle and radius, then offset by the mid-point.
    // Ensure the results fit within the u16 range using max/min bounds and clamp if necessary.
    let x_component = (theta.cos() * r).floor() as i64;
    let y_component = (theta.sin() * r).floor() as i64;

    // Adding components to the mid_point, ensuring the result is within the bounds for u16.
    let x = (mid_point.x as i64 + x_component).clamp(0, u16::MAX as i64) as u16;
    let y = (mid_point.y as i64 + y_component).clamp(0, u16::MAX as i64) as u16;

    Point2d { x, y }
}
