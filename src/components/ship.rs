
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Ship {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
    pub trigger_reset_timer: f32,
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}
