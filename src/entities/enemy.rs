
use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{LocalTransform, Transform};

use super::png_mesh_and_material;

/// Initialises one enemy object in the middle-ish of the arena.
pub fn enemy(world: &mut World) -> Entity {
    let (mesh, background) = png_mesh_and_material("texture/enemy.png", [103.0,84.0], world);

    let mut local_transform = LocalTransform::default();
    local_transform.translation = Vector3::new(30.0, 30.0, 0.0);
    local_transform.scale = Vector3::new(0.1, 0.1, 1.0);

    world
        .create_entity()
        .with(mesh)
        .with(background)
        .with(local_transform)
        .with(Transform::default())
        .build()
}
