//! These are all the custom resources the game uses
//!
//! The difference between an entity and a resource is that:
//!
//! 1. The game usually has multiple copies of an entity, and only one resource
//! 2. Resources can be accessed by the game state and systems whereas entities can only be accessed by systems.
//!
//! The resources used by the space_shooter game are:
//!
//! * **PlayState** the number of lives the player has left
//! * **LaserResource** the texture and material used to create a laser entity on the fly.
//!
//! The resources demonstrate two standard patterns in COP/ECS systems:
//!
//! * Using a resource to transfer information from the entity-creation phase to the system phase (LaserResource) and
//! * Using a resource to transfer information between systems and to the game state (PlayState)

mod play_state;
mod laser;

use amethyst::ecs::prelude::World;

pub use self::play_state::PlayState;
pub use self::laser::LaserResource;

/// Add all the resources needed at the start to the world
/// Note that [laserResource] is not added here, but when the laser component is created.
pub fn add_resources(world: &mut World) {
    world.insert(PlayState{ lives:3});
}