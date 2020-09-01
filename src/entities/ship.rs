//! Manage the ship entity

use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entity, World, WorldExt};
use amethyst::assets::Handle;
use amethyst::prelude::Builder;
use amethyst::renderer::{SpriteSheet, SpriteRender};
use amethyst::window::ScreenDimensions;


// The width and the height come from the png file
const SHIP_WIDTH: f32 = 105.0;
const SHIP_HEIGHT: f32 = 83.0;

use crate::components::Ship;

/// Initialises the player's ship at the bottom centre of the screen
///
/// The function creates a ship sprite (mesh and material),
/// sets up the transform to scale the sprite and to position it
/// at the bottom (y=0) centre of the screen,
/// then finally bundles all the components into an entity.
pub fn initialise_ship(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity {
    let _screen_dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

    // Set the scale and position of our ship sprite
    // so that it's just above the centre bottom of the screen
    let mut local_transform = Transform::default();
    local_transform.set_translation(Vector3::new(
        _screen_dimensions.width() / 2.,
        SHIP_HEIGHT / 2. + 0.1, // add a bit so it's not touching the bottom of the screen
        0.
    ));

    // We also scale the sprite negatively so that we
    // flip it to face up
    local_transform.set_scale(Vector3::new(
        1.,
        -1.,
        1.,
    ));

    // Create a new entity by bundling the mesh, material, component and transforms together
    // then return the entity we created.
    world
        .create_entity()
        .with(Ship {
            velocity: 0.0, // ship starts out stationary
            width: SHIP_WIDTH.into(),
            height: SHIP_HEIGHT.into(),
            trigger_reset_timer: 0.0,
        })
        .with(local_transform)
        .with(SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
        })
        .build()
}
