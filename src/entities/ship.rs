

use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{LocalTransform, Transform};
use amethyst::renderer::ScreenDimensions;

use super::png_mesh_and_material;

use config::GAME_CONFIGURATION;

use components::Ship;

/// Initialises one ball in the middle-ish of the arena.
pub fn initialise_ship(world: &mut World) -> Entity {
    let (mesh, material) = png_mesh_and_material("PNG/playerShip4_purple.png", [105.0,83.0], world);
    let width = {
        let dim = world.read_resource::<ScreenDimensions>();
        dim.width()
    };

    let mut local_transform = LocalTransform::default();
    local_transform.translation = Vector3::new(width/2.0, 0.0, 0.0);
    local_transform.scale = Vector3::new(0.1, 0.1, 1.0);

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(Ship { velocity: GAME_CONFIGURATION.ship_velocity, width: 105.0*0.1, height: 83.0 * 0.1, lives: 3, trigger_reset_timer: 0.0})
        .with(local_transform)
        .with(Transform::default())
        .build()
}
