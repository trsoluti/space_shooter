//! Manage the life entities

use amethyst::assets::Handle;
use amethyst::ecs::prelude::{Entity, World, WorldExt};
use amethyst::prelude::Builder;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::SpriteSheet;
use amethyst::ui::{Anchor, UiImage, UiTransform};

use crate::components::Life;

const LIFE_WIDTH: f32 = 32.;
const LIFE_HEIGHT: f32 = 26.;

/// Initialises the three life entities
///
/// Like [initialise_asteroids](../asteroid/fn.initialise_asteroids.html),
/// this function creates a list of life entities representing the player's lives.
///
/// Note that the number of lives is hard-coded to three. To make it a designer-controlled
/// variable, you can add the item to [game configuration](../../struct.GameConfiguration.html),
/// then modify the code below.
///
/// Note as well that the co-ordination system for the UI layer is different. (0,0) is top-left instead of bottom-left,
/// and the png sprites don't need to be scaled.
pub fn initialise_lives(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
) -> Vec<Entity> {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 3,
    };

    let mut entity_list = Vec::<Entity>::new();
    for i in 0u8..3u8 {
        entity_list.push(
            world
                .create_entity()
                .with(UiTransform::new(
                    format!("life{}", i),
                    Anchor::TopLeft,
                    Anchor::Middle,
                    f32::from(i) * LIFE_WIDTH,
                    -LIFE_HEIGHT,
                    0.,
                    LIFE_WIDTH,
                    LIFE_HEIGHT,
                ))
                .with(UiImage::Sprite(sprite_render.clone()))
                .with(Life { life_number: i })
                .build(),
        );
    }
    entity_list
}
