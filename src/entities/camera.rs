//! Manage the camera entity
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entity, World, WorldExt};
use amethyst::prelude::Builder;
use amethyst::renderer::camera::Camera;
use amethyst::window::ScreenDimensions;

/// Initialises a camera and adds it to the world.
///
/// This game uses an orthographic projection with
/// the lower left corner being (0.0, 0.0)
/// and the upper right corner being (screen_dimensions.width(), screen_dimensions.height())
///
/// Our sprite set happens to be scaled for a world of 1024 x 1024.
pub fn initialise_camera(world: &mut World) -> Entity {
    let screen_dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        screen_dimensions.width() / 2.,
        screen_dimensions.height() / 2.,
        1.
    );
    world
        .create_entity()
        .with(Camera::standard_2d(screen_dimensions.width(), screen_dimensions.height()))
        .with(transform)
        .build()
}
