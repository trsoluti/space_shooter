use components::Asteroid;

use amethyst::core::timing::Time;
use amethyst::core::transform::LocalTransform;
use amethyst::ecs::{Fetch, Join, System, WriteStorage};
use amethyst::renderer::ScreenDimensions;
use rand::thread_rng;

/// This system is responsible for moving the Asteroid
/// and respositioning it if it gets destroyed.
pub struct AsteroidSystem;

impl<'s> System<'s> for AsteroidSystem {
    type SystemData = (
        WriteStorage<'s, Asteroid>,
        WriteStorage<'s, LocalTransform>,
        Fetch<'s, Time>,
        Fetch<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut asteroids, mut transforms, time, screen_dimensions): Self::SystemData) {
        for (asteroid, transform) in (&mut asteroids, &mut transforms).join() {
            transform.translation[1] -= asteroid.velocity * time.delta_seconds();

            // If the asteroid falls below the bottom of the screen,
            // or if it got destroyed in another system,
            // "respawn" it somewhere way up
            // in an ECS, it's more efficient to re-use entities than to
            // destroy and re-create them.
            if asteroid.is_destroyed || transform.translation[1] < (-asteroid.height) {
                let mut rng = thread_rng();
                let local_transform: LocalTransform = asteroid.locate(screen_dimensions.width(), screen_dimensions.height(), &mut rng);
                transform.translation[0] = local_transform.translation[0];
                transform.translation[1] = local_transform.translation[1];
                asteroid.is_destroyed = false;
            }
        }
    }
}

