use components::Ship;
use resources::LaserResource;
use entities::fire_laser;

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
            let optional_movement = input.axis_value("ship");

            let optional_action = input.action_is_down("fire");
            if let Some(action) = optional_action {
                if action {
                    println!("Fire pressed! {}", action);
                    // fire from the middle top of the ship.
                    let mut fire_position = LocalTransform::default();
                    fire_position.translation[0] = transform.translation[0] + ship.width / 2.0 - laser_resource.component.width / 2.0;
                    fire_position.translation[1] = transform.translation[1] + ship.height;
                    fire_position.scale = Vector3{x:0.1, y:0.1, z:1.0};
                    fire_laser(&entities, &laser_resource, fire_position, &lazy_update);
                    /*
                    let laser_entity:Entity = entities.create();
                    lazy_update.insert(laser_entity, laser_resource.material.clone());
                    lazy_update.insert(laser_entity, laser_resource.mesh.clone());
                    lazy_update.insert(laser_entity, laser_resource.component.clone());
                    lazy_update.insert(laser_entity, fire_position.clone());
                    lazy_update.insert(laser_entity, Transform::default());
                    */

                    // TODO: have some sort of countdown timer so can't fire too often.
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

