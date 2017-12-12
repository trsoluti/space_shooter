use components::Ship;

use amethyst::core::timing::Time;
use amethyst::core::transform::LocalTransform;
use amethyst::ecs::{Fetch, Join, System, WriteStorage};
use amethyst::renderer::ScreenDimensions;
use amethyst::input::InputHandler;

/// This system is responsible for moving the ship according to the user
/// provided input.
pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Ship>,
        WriteStorage<'s, LocalTransform>,
        Fetch<'s, Time>,
        Fetch<'s, ScreenDimensions>,
        Fetch<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut ships, mut transforms, time, dimensions, input): Self::SystemData) {
        for (ship, transform) in (&mut ships, &mut transforms).join() {
            let optional_movement = input.axis_value("ship");

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

