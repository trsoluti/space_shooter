//! Manage the camera entity
use amethyst::ecs::{Entity, World};
use amethyst::core::transform::GlobalTransform;
use amethyst::renderer::{Camera, Projection};
use amethyst::core::cgmath::{Matrix4, Vector3};


/// Initialises a camera and adds it to the world.
///
/// This game uses an orthographic projection with
/// the lower left corner being (0.0, 0.0)
/// and the upper right corner being (1024.0, 1024.0)
///
/// Our sprite set happens to be scaled for a world of 1024 x 1024.
pub fn initialise_camera(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Camera::from( Projection::orthographic(
            0.0,
            1024.,
            1024.,
            0.0
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(
                Vector3::new(0.0, 0.0, 1.0).into(),
            )
        ))
        .build()
}
