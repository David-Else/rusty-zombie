use crate::{hero::Hero, zombie::Zombie};
#[derive(Debug)]
pub enum Entity {
    Hero(Hero),
    Zombie(Zombie),
}

#[derive(Debug)]
pub struct GameState {
    pub entities: Vec<Option<Entity>>,
}

impl GameState {
    pub fn new() -> Self {
        let entities: Vec<Option<Entity>> = Vec::new();
        Self { entities }
    }
}
