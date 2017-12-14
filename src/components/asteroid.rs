
use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct Asteroid {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
    pub is_destroyed: bool, // if destroyed, won't respawn
}

impl Component for Asteroid {
    type Storage = DenseVecStorage<Self>;
}
