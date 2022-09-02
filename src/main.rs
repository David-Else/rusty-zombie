mod hero;
mod world;
mod zombie;
use hero::Hero;
use std::io;
use world::Entity;
use world::GameState;
use zombie::Zombie;

// #[...] is an attribute. on Point2d, it generates a Debug trait
// the debug is only needed for this complex type, not simple types
#[derive(Debug)]
pub struct Point2d {
    pub x: usize,
    pub y: usize,
}

fn render_screen(screen: &Vec<Vec<&str>>) {
    // print!("\x1B[2J\x1B[1;1H"); // clear screen

    for linevector in screen.iter() {
        // other way to do it, debug means no blank spaces
        // println!("{:?}", linevector)
        for column in linevector.iter() {
            print!("{}", column)
        }
        println!();
    }
}

fn main() {
    // define the size of the virtual screen
    // will be used for zero based vec, so 15 is 16 spaces
    let width: usize = 15;
    let height: usize = 15;

    // create world, hero and zombie
    // let zombie = Entity::Zombie(Zombie::new(width, height));
    let mut game_state = GameState::new();

    let ze = Option::Some(Entity::Zombie(Zombie::new(width, height)));
    let mut hero = Hero::new(width, height);

    println!("{:?}", ze);
    // push the zombie and hero onto game_state entities vec
    game_state.entities.push(ze);

    //get a the zombie back out so we can get its position...
    let z = game_state.entities.first().unwrap();

    println!("{:?}", z);
    // create 2d array (matrix) as vector to represent the screen
    // " " = empty, "z" = zombie, "h" = hero
    // maybe inefficient https://www.reddit.com/r/rust/comments/fls5v0/can_rust_create_2_dimensional_vector_from_macro/
    let mut screen = vec![vec![" "; width]; height];

    render_screen(&screen);

    loop {
        println!("Press hjkl to move the hero");

        let mut key = String::new();

        io::stdin()
            .read_line(&mut key)
            .expect("Failed to read line");

        hero.keys(key.trim().to_string());
        // put characters on the virtual screen
        screen[hero.position.x][hero.position.y] = "h";
        // screen[zombie.position.x][zombie.position.y] = "z";
        // screen[z.position.x][z.position.y] = "z";
        // hero.keys(String::from("k"));
        render_screen(&screen);
    }
}

// draw the screen in the old way, new is better performance?
// for i in 0..height {
//     println!("{:?}", screen[i])
// }
