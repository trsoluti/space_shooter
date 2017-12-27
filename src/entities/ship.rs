//! Manage the ship entity

use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{LocalTransform, Transform};
use amethyst::renderer::ScreenDimensions;

use super::png_mesh_and_material;

// The width and the height come from the png file
const SHIP_WIDTH:f32 = 105.0;
const SHIP_HEIGHT:f32 = 83.0;
// We scale our sprites at 1/10 so they don't overpower the screen.
const SHIP_DISPLAY_WIDTH:f32 = SHIP_WIDTH * 0.1;
const SHIP_DISPLAY_HEIGHT:f32 = SHIP_HEIGHT * 0.1;


use components::Ship;

/// Initialises the player's ship at the bottom centre of the screen
///
/// The function creates a ship sprite (mesh and material),
/// sets up the transform to scale the sprite and to position it
/// at the bottom (y=0) centre of the screen,
/// then finally bundles all the components into an entity.
pub fn initialise_ship(world: &mut World) -> Entity {
    let (mesh, material) = png_mesh_and_material(
        "PNG/playerShip4_purple.png",
        [SHIP_WIDTH, SHIP_HEIGHT], world);
    let width = {
        let dim = world.read_resource::<ScreenDimensions>();
        dim.width()
    };

    // Set the scale and position of our ship sprite:
    let mut local_transform = LocalTransform::default();
    local_transform.translation = Vector3::new(width/2.0 - SHIP_DISPLAY_WIDTH/2.0, 0.0, 0.0);
    local_transform.scale = Vector3::new(0.1, 0.1, 1.0);

    // Create a new entity by bundling the mesh, material, component and transforms together
    // then return the entity we created.
    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(Ship {
            velocity: 0.0, // ship starts out stationary
            width: SHIP_DISPLAY_WIDTH,
            height: SHIP_DISPLAY_HEIGHT,
            trigger_reset_timer: 0.0})
        .with(local_transform)
        .with(Transform::default())
        .build()
}
