use crate::{hero::HeroMove, world::Entity, Point2d};
#[derive(Debug)]
pub struct Zombie {
    pub position: Point2d,
}

// fn calculate_random_position_around_point(
//     hero_position: Point2d,
//     screen_width_height: Point2d,
// ) -> Point2d {
//     let minimum_r = screen_width_height.y / 2;

//     let rn: f64 = thread_rng().gen(); //.gen_range(0..1);
//     let theta = rn * (2.0 * PI) as f64;

//     let r: f64 =
//         (thread_rng().gen_range((((minimum_r as f64) / 2.0).floor()) as usize..minimum_r)) as f64; // * (variation_in_r + minimum_r) as f64;

//     Point2d {
//         x: (((theta.cos() * r).floor() as isize) + (hero_position.x) as isize) as usize,
//         y: (((theta.sin() * r).floor() as isize) + (hero_position.y) as isize) as usize,
//     }
// }

impl Entity for Zombie {
    // associated (static) function, used as constructor
    fn new(random_position: Point2d) -> Self {
        Self {
            position: random_position,
        }
    }
    fn update(&mut self, _key: HeroMove) {
        // ERROR unuses str to match trait, how fix?
        println!("I am updating")
    }
}
