extern crate amethyst;

mod state;
mod components;
mod entities;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DisplayConfig, RenderBundle, RenderSystem, Pipeline, Stage, DrawFlat, PosTex};
use amethyst::ui::{DrawUi, UiBundle};

use state::SpriteState;

const BACKGROUND_COLOUR: [f32; 4] = [16.0, 16.0, 16.0, 0.0]; // dark grey

fn run() -> Result<(), amethyst::Error> {
    let display_config_path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let display_config = DisplayConfig::load(&display_config_path);

    let resources_path = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));

    let game = Application::build(
        resources_path,
        SpriteState)?
        .with_bundle(RenderBundle::new())?
        .with_bundle(UiBundle::new())?
        .with_bundle(TransformBundle::new())?;

    let pipe = {
        let loader = game.world.read_resource();
        let mesh_storage = game.world.read_resource();
        Pipeline::build()
            .with_stage(
                Stage::with_backbuffer()
                    .clear_target(BACKGROUND_COLOUR, 1.0)
                    .with_pass(DrawFlat::<PosTex>::new())
                    .with_pass(DrawUi::new(&loader, &mesh_storage))
            )
    };

    Ok(
        game.with_local(RenderSystem::build(pipe, Some(display_config))?)
            .build()?
            .run(),
    )
}


fn main() {
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}
