
use amethyst::ecs::prelude::{Component, DenseVecStorage};

/// A component for our asteroid
///
/// We store the velocity, width and height with the asteroid
/// even though they are constants, to make them readily
/// available.
///
/// In a proper COP design, you only store what will be
/// modified by a system.
#[derive(Clone)]
pub struct Asteroid {
    /// How fast the asteroid is falling
    pub velocity: f32,
    /// The width of the asteroid in pixels
    pub width: f32,
    /// The height of the asteroid in pixels
    pub height: f32,
    /// Whether or not the asteroid has been destroyed and is ready for relocation
    pub is_destroyed: bool,
}

impl Component for Asteroid {
    type Storage = DenseVecStorage<Self>;
}
