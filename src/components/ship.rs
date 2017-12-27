
use amethyst::ecs::{Component, DenseVecStorage};

/// The component for the player's space ship
///
/// We store the width and height with the ship
/// even though they are constants, to make them readily
/// available.
///
/// In a proper COP design, you only store what will be
/// modified by a system.
pub struct Ship {
    /// Current horizontal speed of the ship
    pub velocity: f32,
    /// The width of the ship sprite
    pub width: f32,
    /// The height of the ship sprite
    pub height: f32,
    /// How much time in seconds has passed since the last time the laser was fired
    pub trigger_reset_timer: f32,
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}
