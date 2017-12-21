
use amethyst::core::transform::LocalTransform;
use amethyst::ecs::{Entities, Join, System, WriteStorage, Fetch};
use amethyst::renderer::ScreenDimensions;
use amethyst::core::timing::Time;

use components::Laser;
use config::GAME_CONFIGURATION;

pub struct LaserSystem;

impl<'s> System<'s> for LaserSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Laser>,
        WriteStorage<'s, LocalTransform>,
        Fetch<'s, ScreenDimensions>,
        Fetch<'s, Time>,
    );

    fn run(&mut self, (entities, mut lasers, mut transforms, screen_dimensions, time): Self::SystemData) {
        // Scan through the list of lasers and move them forward.
        for (laser_entity, _laser_component, laser_transform) in (&*entities, &mut lasers, &mut transforms).join() {
            laser_transform.translation[1] += GAME_CONFIGURATION.laser_velocity * time.delta_seconds();

            // Delete the laser if it has gone off the screen
            if laser_transform.translation[1] > screen_dimensions.height() {
                let _result = entities.delete(laser_entity);
            }
        }
    }
}