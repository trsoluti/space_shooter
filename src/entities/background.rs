//! Manage the background entity

use amethyst::ecs::prelude::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{Transform, GlobalTransform};

use super::png_mesh_and_material;

/// Initialises the background as a sprite object
pub fn initialise_background(world: &mut World) -> Entity {

    let (mesh, background) = png_mesh_and_material("Backgrounds/darkPurple.png", [1024.0, 1024.0], world);

    let mut local_transform = Transform::default();
    local_transform.translation = Vector3::new(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(mesh)
        .with(background)
        .with(local_transform)
        .with(GlobalTransform::default())
        .build()
}

