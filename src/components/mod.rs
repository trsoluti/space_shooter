//! Components for the game
//!
//! Each struct in this module represents a component of our space_shooter game.
//!
//! Note all components should be cloneable, so you can have more copies of
//! a component when you need them. That's just good practice.

mod asteroid;
mod laser;
mod life;
mod ship;

use amethyst::ecs::prelude::World;
use amethyst::ecs::WorldExt;

pub use self::asteroid::Asteroid;
pub use self::laser::Laser;
pub use self::life::Life;
pub use self::ship::Ship;

/// Register all the components to the world
pub fn register_components(world: &mut World) {
    world.register::<Ship>();
    world.register::<Asteroid>();
    world.register::<Laser>();
    world.register::<Life>();
}
