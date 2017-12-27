//! Manage the camera entity
use amethyst::ecs::{Entity, World};
use amethyst::core::transform::Transform;
use amethyst::renderer::{Camera, ScreenDimensions, Projection};
use amethyst::core::cgmath::{Matrix4, Vector3};


/// Initialises a camera and adds it to the world.
///
/// The camera uses orthographic projection, which is common
/// for 2D sprite-driven applications.
///
/// It calculates its shape based on the screen dimensions
/// at the time the camera was created.
///
/// The pattern to get the screen dimensions means that you
/// borrow the resource just long enough to pull out the
/// (non-reference) width and height. Otherwise you will
/// get error messages about borrowing the world twice.
pub fn initialise_camera(world: &mut World) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    world
        .create_entity()
        .with(Camera::from( Projection::orthographic(
            0.0,
            width,
            height,
            0.0
        )))
        .with(Transform(
            Matrix4::from_translation(
                Vector3::new(0.0, 0.0, 1.0).into(),
            )
        ))
        .build()
}
