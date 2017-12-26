use amethyst::ecs::{Entity, World};
use amethyst::renderer::{PngFormat, Texture};
use amethyst::ui::{UiImage, UiTransform};
use amethyst::assets::{AssetStorage, Loader};

use components::Life;

const LIFE_WIDTH:f32 = 32.;
const LIFE_HEIGHT:f32 = 26.;

pub fn initialise_lives(world: &mut World) -> [Entity; 3] {
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
    [ entity_list[0], entity_list[1], entity_list[2] ]
}