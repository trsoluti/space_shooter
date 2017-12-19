
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Laser {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
}

impl Component for Laser {
    type Storage = DenseVecStorage<Self>;
}