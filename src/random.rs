use crate::Point2d;
use rand::{thread_rng, Rng};

pub fn random_usize_in_inclusive_range(min: usize, max: usize) -> usize {
    thread_rng().gen_range(min..=max)
}

pub fn random_direction() -> usize {
    random_usize_in_inclusive_range(0, 3)
}

pub fn random_position_around_point(screen_size: Point2d) -> Point2d {
    let mid_point = Point2d {
        x: screen_size.x / 2,
        y: screen_size.y / 2,
    };

    let minimum_r = screen_size.x / 2;

    let rn: f64 = thread_rng().gen(); //.gen_range(0..1);
    let theta = rn * (2.0 * std::f32::consts::PI) as f64;

    let r: f64 =
        (thread_rng().gen_range((((minimum_r as f64) / 2.0).floor()) as usize..minimum_r)) as f64; // * (variation_in_r + minimum_r) as f64;

    Point2d {
        x: (((theta.cos() * r).floor() as isize) + (mid_point.x) as isize) as usize,
        y: (((theta.sin() * r).floor() as isize) + (mid_point.y) as isize) as usize,
    }
}
