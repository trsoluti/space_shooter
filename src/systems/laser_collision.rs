
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entities, Join, System, ReadStorage, WriteStorage};

use crate::components::Laser;
use crate::components::Asteroid;

/// Removes the laser and repositions the asteroid
/// if it detects a collision between them
pub struct LaserCollisionSystem;

impl<'s> System<'s> for LaserCollisionSystem {
    /// The data for each pass of the laser collision system
    /// We need:
    ///
    /// * **Entities**:   the list of entities so we can delete the laser
    ///                     when it collides with an asteroid
    /// * **Lasers**:     read access to the list of laser components
    ///                     so we select only the laser entities and transforms
    /// * **Transforms**: read access to the list of transforms
    ///                     so we can determine the laser and asteroid positions
    /// * **Asteroids**:  write access to the list of asteroids
    ///                     so we can mark an asteroid for repositioning
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Laser>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Asteroid>,
    );

    /// Runs a pass of the system on our selected components.
    ///
    /// This function is given a list of the components described under [SystemData](#associatedtype.SystemData).
    ///
    /// It first selects every laser component with its entity and transformation.
    /// For each laser, it scans every asteroid with its location.
    /// For each asteroid, it determines if the two items have collided.
    /// If they have, the function deletes the laser and marks the asteroid for repositioning.
    fn run(&mut self, (entities, lasers, transforms, mut asteroids): Self::SystemData) {
        // For each laser,
        for (laser_entity, laser_component, laser_transform) in (&*entities, &lasers, &transforms).join() {
            // Set up the collision box for our laser:
            let laser_left = laser_transform.translation()[0];
            let laser_right = laser_left + laser_component.width;
            let laser_top = laser_transform.translation()[1] + laser_component.height;

            // scan our asteroids to see if we have hit any one of them
            for (asteroid_component, asteroid_transform) in (&mut asteroids, &transforms).join() {
                // Set up a collision box for our asteroid
                let asteroid_left = asteroid_transform.translation()[0];
                let asteroid_bottom = asteroid_transform.translation()[1];
                let asteroid_right = asteroid_left + asteroid_component.width;

                // If the two items overlap,
                if ((laser_left <= asteroid_right && laser_left >= asteroid_left)
                    || (laser_right <= asteroid_left && laser_right >= asteroid_right))
                    && (laser_top >= asteroid_bottom) {
                    // we have a collision. Delete the laser
                    let _result = entities.delete(laser_entity);
                    // let the asteroid system know the asteroid is ready for respawn/relocation
                    asteroid_component.is_destroyed = true;
                }
            }
        }
    }
}