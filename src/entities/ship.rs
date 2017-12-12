

use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{LocalTransform, Transform};
use amethyst::renderer::ScreenDimensions;

use super::png_mesh_and_material;
use config::SHIP_VELOCITY;

use components::Ship;

/// Initialises one ball in the middle-ish of the arena.
pub fn initialise_ship(world: &mut World) -> Entity {
    let (mesh, background) = png_mesh_and_material("texture/spaceship.png", [103.0,84.0], world);
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut local_transform = LocalTransform::default();
    local_transform.translation = Vector3::new(width/2.0, height-8.4, 0.0);
    local_transform.scale = Vector3::new(0.1, 0.1, 1.0);

    world
        .create_entity()
        .with(mesh)
        .with(background)
        .with(Ship { velocity: SHIP_VELOCITY, width: 103.0*0.1, height: 84.0 * 0.1})
        .with(local_transform)
        .with(Transform::default())
        .build()
}
