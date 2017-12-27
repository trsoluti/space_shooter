use amethyst::core::timing::Time;
use amethyst::core::transform::LocalTransform;
use amethyst::ecs::{Fetch, Join, System, WriteStorage};
use amethyst::renderer::ScreenDimensions;
use rand::thread_rng;

use components::Asteroid;
use entities::locate_asteroid;

/// Moves the asteroid, either down by its velocity
/// or to a new random location if it was marked for repositioning.
pub struct AsteroidSystem;

impl<'s> System<'s> for AsteroidSystem {
    /// The data for each pass of the asteroid system
    /// We need:
    ///
    /// * **Asteroids**:         write access to the list of asteroid components
    ///                            so we can turn off the "destroyed" flag once the asteroid is relocated.
    /// * **Transforms**:        write access to the list of positions
    ///                            so we can update the asteroid's position
    /// * **Time**:              read access to the time resource so we can know how much time
    ///                            has elapsed since we last ran this system
    /// * **Screen Dimensions**: read access to the screen dimensions
    ///                            so we can reposition the asteroid properly
    type SystemData = (
        WriteStorage<'s, Asteroid>,
        WriteStorage<'s, LocalTransform>,
        Fetch<'s, Time>,
        Fetch<'s, ScreenDimensions>,
    );

    /// Runs a pass of the system on our selected components
    ///
    /// This function is given a list of the components described under [SystemData](#associatedtype.SystemData).
    ///
    /// It selects all the asteroids with their prospective locations from the list.
    /// For each asteroid, it updates the asteroid's position.
    ///
    /// If the asteroid has fallen belows the screen or is marked for respawn/relocation,
    /// It calls the [locate_asteroid](../entities/fn.locate_asteroid.html) function
    /// to determine a new position for the asteroid, and moves it there.
    fn run(&mut self, (mut asteroids, mut transforms, time, screen_dimensions): Self::SystemData) {
        for (asteroid, transform) in (&mut asteroids, &mut transforms).join() {
            // move the asteroid by its velocity
            transform.translation[1] -= asteroid.velocity * time.delta_seconds();

            // If the asteroid falls below the bottom of the screen,
            // or if it got destroyed in another system,
            // "respawn" it somewhere way up
            // in an ECS, it's more efficient to re-use entities than to
            // destroy and re-create them.
            if asteroid.is_destroyed || transform.translation[1] < (-asteroid.height) {
                let mut rng = thread_rng();
                // TODO: figure out why dimensions in systems are 5 times as big
                let local_transform: LocalTransform = locate_asteroid(asteroid, screen_dimensions.width()/5., screen_dimensions.height()/5., &mut rng);
                transform.translation[0] = local_transform.translation[0];
                transform.translation[1] = local_transform.translation[1];
                asteroid.is_destroyed = false;
            }
        }
    }
}

