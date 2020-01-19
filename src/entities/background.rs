//! Manage the background entity

use amethyst::prelude::Builder;
use amethyst::ecs::prelude::{Entity, World, WorldExt};
use amethyst::core::transform::{Transform};

use super::png_mesh_and_material;

/// Initialises the background as a sprite object
pub fn initialise_background(world: &mut World) -> Entity {

    let (mesh, background) = png_mesh_and_material("Backgrounds/darkPurple.png", [1024.0, 1024.0], world);

    let local_transform = Transform::default();

    world
        .create_entity()
        .with(mesh)
        .with(background)
        .with(local_transform)
        .build()
}

