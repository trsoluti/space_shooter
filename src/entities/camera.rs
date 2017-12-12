
use amethyst::ecs::{Entity, World};
use amethyst::core::transform::Transform;
use amethyst::renderer::{Camera, ScreenDimensions, Projection};
use amethyst::core::cgmath::{Matrix4, Vector3};


/// This function initialises a camera and adds it to the world.
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
