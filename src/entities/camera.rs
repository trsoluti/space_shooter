//! Manage the camera entity
use amethyst::prelude::Builder;
use amethyst::ecs::prelude::{Entity, World, WorldExt};
use amethyst::core::transform::Transform;
use amethyst::renderer::camera::{Camera, Projection};



/// Initialises a camera and adds it to the world.
///
/// This game uses an orthographic projection with
/// the lower left corner being (0.0, 0.0)
/// and the upper right corner being (1024.0, 1024.0)
///
/// Our sprite set happens to be scaled for a world of 1024 x 1024.
pub fn initialise_camera(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(Camera::from( Projection::orthographic(
            0.0,
            1024.,
            0.0,
            1024.,
            0.1,
            2000.0,
        )))
        .with(transform)
        .build()
}
