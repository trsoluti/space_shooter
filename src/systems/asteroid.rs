use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Read, Join, System, WriteStorage};
use rand::thread_rng;

use crate::components::Asteroid;
use crate::entities::locate_asteroid;

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
    type SystemData = (
        WriteStorage<'s, Asteroid>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
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
    fn run(&mut self, (mut asteroids, mut transforms, time): Self::SystemData) {
        for (asteroid, transform) in (&mut asteroids, &mut transforms).join() {
            let (screen_width, screen_height) = {
                (1024., 1024.)
            };
            // move the asteroid by its velocity
            transform.prepend_translation_y(-asteroid.velocity * time.delta_seconds());

            // If the asteroid falls below the bottom of the screen,
            // or if it got destroyed in another system,
            // "respawn" it somewhere way up
            // in an ECS, it's more efficient to re-use entities than to
            // destroy and re-create them.
            if asteroid.is_destroyed || transform.translation()[1] < (0. - (screen_height+asteroid.height)).into() {
                let mut rng = thread_rng();
                let local_transform = locate_asteroid(asteroid, screen_width, screen_height, &mut rng);
                transform.set_translation_x(local_transform.translation()[0]);
                transform.set_translation_y(local_transform.translation()[1]);
                asteroid.is_destroyed = false;
            }
        }
    }
}

