use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, ReadStorage, System, Write, WriteStorage};

use crate::components::Asteroid;
use crate::components::Ship;
use crate::resources::PlayState;

/// Reduces the number of lives
/// and marks the asteroid for repositioning
/// if it detects a ship colliding with an asteroid.
pub struct ShipCollisionSystem;

impl<'s> System<'s> for ShipCollisionSystem {
    /// The data for each pass of the ship collision system
    /// We need:
    ///
    /// * **PlayState**:  write access to the play state
    ///                     so we can update the number of lives
    /// * **Ships**:      read access to the list of ships (which consists of exactly one ship)
    ///                     so we can determine the ship's collision box
    /// * **Transforms**: read access to the list of ship and asteroid locations
    ///                     so we can determine both collision boxes
    /// * **Asteroids**:  write access to the list of asteroids
    ///                     so we can mark an asteroid for repositioning
    ///
    /// Note that we have a list of ships even though the game has only one ship.
    /// This is the appropriate way to extract the ship and transport component from our storage.
    type SystemData = (
        Write<'s, PlayState>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Asteroid>,
    );

    /// Runs a pass of the system on our selected components.
    ///
    /// This function is given a list of the components described under [SystemData](#associatedtype.SystemData).
    /// It first runs a pass on every ship with its location (don't worry that we have only one ship).
    ///
    /// For each ship, it then runs another pass on each asteroid with its location.
    /// For each asteroid, it checks whether or not the asteroid overlaps with the ship.
    /// If it does, it marks the asteroid for repositioning.
    ///
    /// The [AsteroidSystem](struct.AsteroidSystem.html) will do the actual repositioning,
    /// since it has write access to the transform list.
    ///
    /// Note the second pass ("join") will be run in parallel for maximum throughput. This
    /// is the key advantage of an Entity-Component System.
    fn run(&mut self, (mut play_state, ships, transforms, mut asteroids): Self::SystemData) {
        for (ship_component, ship_transform) in (&ships, &transforms).join() {
            // create a collision box for our ship
            let ship_left = ship_transform.translation()[0];
            let ship_top = ship_transform.translation()[1] + ship_component.height;
            let ship_right = ship_left + ship_component.width;

            // check to see if our ship has collided with any asteroid
            for (asteroid_component, asteroid_transform) in (&mut asteroids, &transforms).join() {
                // create a collision box for our asteroid
                let asteroid_left = asteroid_transform.translation()[0];
                let asteroid_bottom = asteroid_transform.translation()[1];
                let asteroid_right = asteroid_left + asteroid_component.width;

                // if the two collision boxes overlap,
                if ((ship_left <= asteroid_right && ship_left >= asteroid_left)
                    || (ship_right <= asteroid_left && ship_right >= asteroid_right))
                    && (ship_top >= asteroid_bottom)
                {
                    // we have a collision. Decrement the number of lives of the game
                    if play_state.lives > 0 {
                        play_state.lives -= 1;
                    }
                    // let the asteroid system know the asteroid is ready for respawn/relocation
                    asteroid_component.is_destroyed = true;
                }
            }
        }
    }
}
