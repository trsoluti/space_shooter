
use amethyst::core::transform::LocalTransform;
use amethyst::ecs::{Entities, Join, System, ReadStorage, WriteStorage};

use components::Laser;
use components::Asteroid;

pub struct LaserCollisionSystem;

impl<'s> System<'s> for LaserCollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Laser>,
        ReadStorage<'s, LocalTransform>,
        WriteStorage<'s, Asteroid>,
    );

    fn run(&mut self, (entities, mut lasers, transforms, mut asteroids): Self::SystemData) {
        // For each laser,
        for (laser_entity, laser_component, laser_transform) in (&*entities, &mut lasers, &transforms).join() {
            // Set up the collision box for our laser:
            let laser_left = laser_transform.translation[0];
            let laser_right = laser_left + laser_component.width;
            let laser_top = laser_transform.translation[1] + laser_component.height;

            // scan our asteroids to see if we have hit any one of them
            for (asteroid_component, asteroid_transform) in (&mut asteroids, &transforms).join() {
                let asteroid_left = asteroid_transform.translation[0];
                let asteroid_bottom = asteroid_transform.translation[1];
                let asteroid_right = asteroid_left + asteroid_component.width;

                if ((laser_left <= asteroid_right && laser_left >= asteroid_left)
                    || (laser_right <= asteroid_left && laser_right >= asteroid_right))
                    && (laser_top >= asteroid_bottom) {
                    // we have a collision. Delete the laser
                    let _result = entities.delete(laser_entity);
                    // let the respawn system know the asteroid is destroyed and ready for respawn
                    asteroid_component.is_destroyed = true;
                }
            }
        }
    }
}