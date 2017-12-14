

use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};

use components::Ship;
use components::Asteroid;
use systems::ShipSystem;
use systems::AsteroidSystem;

/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the world for a game of pong.
pub struct GameBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for GameBundle {
    fn build(self, world: &mut World, builder: DispatcherBuilder<'a, 'b>) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register::<Ship>();
        world.register::<Asteroid>();

        Ok(
            builder
                .add(ShipSystem, "ship_system", &["input_system"])
                .add(AsteroidSystem, "asteroid_system", &["ship_system"])
        )
    }
}