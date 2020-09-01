//! Manage the background entity

use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entity, World, WorldExt};
use amethyst::prelude::Builder;

use amethyst::assets::AssetStorage;
use amethyst::assets::Handle;
use amethyst::assets::Loader;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::Sprite;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::Texture;
use amethyst::window::ScreenDimensions;

/// Initialises the background as a sprite object
///
/// Sprites are assumed to belong to some "sprite sheet"
/// which stores a list of sprites.
pub fn initialise_background(world: &mut World) -> Entity {
    let sprite_sheet_handle = load_background_sprite_sheet_handle(world);
    let screen_dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    // Position the centre of the background sprite to be
    // the centre of the screen.
    let mut local_transform = Transform::default();
    local_transform.set_translation( Vector3::new(
        screen_dimensions.height() / 2.,
        screen_dimensions.width() / 2.,
        0.
    ));

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .build()
}

/// Loads a handle for the 'sprite sheet' for our background
///
/// A sprite sheet defines the position and size of a list of sprites
/// within a texture file,
/// as well as the size to display each sprite on the screen.
///
/// Normally these will be loaded through a sprite sheet definition file,
/// but that file does not offer the capability of 'scaling' the sprite
/// when we load it.
fn load_background_sprite_sheet_handle(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = load_background_texture(world);
    let sprite_count = 1;
    let mut sprites = Vec::with_capacity(sprite_count);

    let sprite_width: f32 = 1024.; // width of our background sprite
    let sprite_height: f32 = 1024.; // height of our background sprite

    let dimensions = (sprite_width, sprite_height);
    let texture_coordinates: [f32; 4] = [
        /* left:   */ 0., /* right:  */ 1., /* bottom: */ 0., /* top:    */ 1.,
    ]; // i.e. the whole png image

    sprites.push(Sprite::from((dimensions, texture_coordinates)));

    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(
        SpriteSheet {
            texture: texture_handle,
            sprites,
        },
        (),
        &sprite_sheet_storage,
    )
}

/// Loads the texture (sprite image) of the background
/// and returns a handle to it, which can then be used
/// to create a sprite sheet.
fn load_background_texture(world: &mut World) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        "Backgrounds/darkPurple.png",
        ImageFormat::default(),
        (),
        &texture_storage,
    )
}
