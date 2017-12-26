

use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};

use components::*;
use systems::*;
use resources::{PlayState};

/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the world for a game of pong.
pub struct GameBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for GameBundle {
    fn build(self, world: &mut World, builder: DispatcherBuilder<'a, 'b>) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register::<Ship>();
        world.register::<Asteroid>();
        world.register::<Laser>();
        world.register::<Life>();
        world.add_resource(PlayState{ lives:3});

        Ok(
            builder
                .add(ShipSystem, "ship_system", &["input_system"])
                .add(CollisionSystem, "collision_system", &["ship_system"])
                .add(AsteroidSystem, "asteroid_system", &["collision_system"])
                .add(LaserSystem, "laser_system", &["ship_system"])
                .add(LaserCollisionSystem, "laser_collision_system", &["laser_system"])
                .add(LivesSystem, "lives_system", &["collision_system"])
        )
    }
}