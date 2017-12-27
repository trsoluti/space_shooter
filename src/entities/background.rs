//! Manage the background entity

use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{LocalTransform, Transform};

use super::png_mesh_and_material;

/// Initialises the background as a sprite object
pub fn initialise_background(world: &mut World) -> Entity {

    let (mesh, background) = png_mesh_and_material("Backgrounds/darkPurple.png", [1024.0, 1024.0], world);

    let mut local_transform = LocalTransform::default();
    local_transform.translation = Vector3::new(0.0, 0.0, 0.0);
    local_transform.scale = Vector3::new(0.1, 0.1, 1.0);

    world
        .create_entity()
        .with(mesh)
        .with(background)
        .with(local_transform)
        .with(Transform::default())
        .build()
}

