use components::Ship;
use resources::LaserResource;
use entities::fire_laser;
use config::GAME_CONFIGURATION;

use amethyst::core::cgmath::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::transform::LocalTransform;
use amethyst::ecs::{Fetch, Join, System, WriteStorage, Entities, LazyUpdate};
use amethyst::renderer::ScreenDimensions;
use amethyst::input::InputHandler;

/// This system is responsible for moving the ship according to the user
/// provided input.
pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ship>,
        WriteStorage<'s, LocalTransform>,
        Fetch<'s, Time>,
        Fetch<'s, ScreenDimensions>,
        Fetch<'s, InputHandler<String, String>>,
        Fetch<'s, LaserResource>,
        Fetch<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut ships, mut transforms, time, dimensions, input, laser_resource, lazy_update): Self::SystemData) {
        for (ship, transform) in (&mut ships, &mut transforms).join() {
            // count down on the amount of time before we can fire again.
            if ship.trigger_reset_timer > 0.0 {
                ship.trigger_reset_timer -= time.delta_seconds();
            }
            let optional_movement = input.axis_value("ship");

            let optional_action = input.action_is_down("fire");
            if let Some(action) = optional_action {
                if action && ship.trigger_reset_timer <= 0.0 {
                    // fire from the middle top of the ship.
                    let fire_position = Vector3{
                        x: transform.translation[0] + ship.width / 2.0,
                        y: transform.translation[1] + ship.height,
                        z: 0.0,
                    };
                    fire_laser(&entities, &laser_resource, fire_position, &lazy_update);

                    ship.trigger_reset_timer = GAME_CONFIGURATION.trigger_reset_timeout;
                }
            }

            if let Some(movement) = optional_movement {
                let arena_width = dimensions.width();
                transform.translation[0] -= ship.velocity * time.delta_seconds() * movement as f32;

                transform.translation[0] = transform.translation[0]
                    .max(0.0)
                    .min(arena_width - ship.width)
            }
        }
    }
}

