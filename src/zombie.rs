use crate::{world::Entity, Point2d};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Zombie {
    pub position: Point2d,
    image: char,
}

pub fn calculate_random_position_around_point(
    hero_position: Point2d,
    screen_width_height: Point2d,
) -> Point2d {
    let variation_in_r = 1;
    let minimum_r = screen_width_height.y / 2; // could be x
    let theta = thread_rng().gen_range(0..1) as f32 * (2.0 * PI);
    let r = thread_rng().gen_range(0..1) * variation_in_r + minimum_r;
    Point2d {
        x: theta.cos() as usize * r + hero_position.x,
        y: theta.sin() as usize * r + hero_position.y,
    }
}

impl Entity for Zombie {
    // associated (static) function, used as constructor
    fn new(screen_size: Point2d, image: char) -> Self {
        let random_position = Point2d {
            x: thread_rng().gen_range(0..screen_size.x),
            y: thread_rng().gen_range(0..screen_size.y),
        };
        // --- TEST --- WHY does this not work?
        let rando = calculate_random_position_around_point(
            Point2d {
                x: screen_size.x / 2,
                y: screen_size.y / 2,
            },
            screen_size,
        );
        println!("NEW ALGO {:?}", rando);
        println!("OLD ALGO {:?}", random_position);
        // ---------------------------------------
        Self {
            position: random_position,
            image,
        }
    }
    fn update(&mut self, key: &str) {
        // ERROR unuses str to match trait, how fix?
        println!("I am updating")
    }
}

// export const calculateRandomPositionAroundPoint: VectorFn = (
//   centrePoint,
//   screenWidthHeight
// ) => {
//   const variationInR = 1;
//   const minimumR = screenWidthHeight[1] / 2; // TODO add size of entity make always on screen
//   const theta = Math.random() * (2 * Math.PI);
//   const r = Math.random() * variationInR + minimumR;
//   return [
//     Math.cos(theta) * r + centrePoint[0],
//     Math.sin(theta) * r + centrePoint[1],
//   ];
// };
