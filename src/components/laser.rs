
use amethyst::ecs::{Component, DenseVecStorage};

/// Component for our asteroid-destroying laser beam.
#[derive(Clone)]
pub struct Laser {
    /// The speed of our laser beam
    pub velocity: f32,
    /// The width of our laser beam
    pub width: f32,
    /// The height of our laser beam
    pub height: f32,
}

impl Component for Laser {
    type Storage = DenseVecStorage<Self>;
}