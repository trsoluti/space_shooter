

use amethyst::core::bundle::{SystemBundle, Result};
use amethyst::ecs::prelude::{DispatcherBuilder};

//use components::*;
use crate::systems::*;
//use resources::{PlayState};

/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world.
///
/// Note that all components and resources need to be registered in the world before they
/// can be referenced in a system or in the state.
///
/// This bundle prepares the world for the space_shooter game.
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(ShipSystem, "ship_system", &["input_system"]);
        builder.add(ShipCollisionSystem, "collision_system", &["ship_system"]);
        builder.add(AsteroidSystem, "asteroid_system", &["collision_system"]);
        builder.add(LaserSystem, "laser_system", &["ship_system"]);
        builder.add(LaserCollisionSystem, "laser_collision_system", &["laser_system"]);
        builder.add(LivesSystem, "lives_system", &["collision_system"]);
        Ok(())
    }
}