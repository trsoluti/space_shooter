//! Manage the life entities

use amethyst::ecs::{Entity, World};
use amethyst::renderer::{PngFormat, Texture};
use amethyst::ui::{UiImage, UiTransform};
use amethyst::assets::{AssetStorage, Loader};

use components::Life;

const LIFE_WIDTH:f32 = 32.;
const LIFE_HEIGHT:f32 = 26.;

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
pub fn initialise_lives(world: &mut World) -> Vec<Entity> {
    let logo = {
        let loader = world.read_resource::<Loader>();
        let life_texture = loader.load(
            "PNG/UI/playerLife1_blue.png",
            PngFormat,
            Default::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        );
        life_texture
    };

    let mut entity_list = Vec::<Entity>::new();
    for i in 0u8..3u8 {
        entity_list.push(
            world
                .create_entity()
                .with(UiTransform::new(
                    format!("life{}",i),
                    f32::from(i) * LIFE_WIDTH,
                    0., // UI is 0-top, whereas regular is 0-bottom
                    0.,
                    LIFE_WIDTH,
                    LIFE_HEIGHT,
                    0,
                ))
                .with(UiImage {
                    texture: logo.clone(),
                })
                .with(Life{life_number:i})
                .build()
        );
    }
    entity_list
}