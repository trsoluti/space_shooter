//! A simple space shooter game using Component-Oriented Programming (COP) and Amethyst
//!
//! Each part of the game is documented, providing patterns you can use
//! to solve various problems in creating games that run under Amethyst.
//!
//! To summarise a Component-Oriented Programming model (also called Entity-Component System or ECS model),
//! a product has the following classes of items:
//!
//! <table>
//!   <tr>
//!     <th>Class</th>
//!     <th>Role</th>
//!   </tr>
//!   <tr>
//!     <td><a href="components/index.html">components</a></td>
//!     <td>a basic game object, with the data specific to it.</td>
//!   </tr>
//!   <tr>
//!     <td><a href="entities/index.html">entities</a></td>
//!     <td>collection of related components e.g. a game object, its location, material, mesh, etc.</td>
//!   </tr>
//!   <tr>
//!     <td><a href="systems/index.html">systems</a></td>
//!     <td>the sets of rules that act on entities by changing their data.</td>
//!   </tr>
//!   <tr>
//!     <td>resources</td>
//!     <td>data "global" to the game that the entity needs, e.g. the screen bounds.</td>
//!   </tr>
//! </table>
//!
//! In our example, each of those items are put in a separate module.
//!
//! In an Amethyst game, there are three other common elements:
//!
//! <table>
//!   <tr>
//!    <th>Item</th>
//!    <th>Role</th>
//!   </tr>
//!   <tr>
//!     <td><a href="struct.GameState.html">game state</a></td>
//!     <td>Actions to take at the start of the game, on each cycle, and at the end of the game.</td>
//!   </tr>
//!   <tr>
//!     <td><a href="struct.GameBundle.html">game bundle</a></td>
//!     <td>The collection of components, resources and systems that make up the game.</td>
//!   </tr>
//!   <tr>
//!     <td><a href="struct.GameConfiguration.html">game configuration</td>
//!     <td>The set of items used by the Game Designer to tune the game after the coding is complete (e.g. player speed)</td>
//!   </tr>
//! </table>
//!
//! These items are put in their own Rust files.

#![deny(missing_docs)]

extern crate amethyst;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod config;
mod state;
mod bundle;
pub mod components;
pub mod entities;
pub mod systems;
mod resources;

// public use so these things get documented
pub use config::GameConfiguration;
pub use config::GAME_CONFIGURATION;
pub use state::GameState;
pub use bundle::GameBundle;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DisplayConfig, RenderBundle, RenderSystem, Pipeline, Stage, DrawFlat, PosTex};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::input::InputBundle;



const BACKGROUND_COLOUR: [f32; 4] = [0.25, 0.25, 0.25, 0.0]; // dark grey

/// Run the game
///
/// This function does the following:
///
/// 1. Loads up the display configuration and input bindings from RON files
///    in the resources folder;
/// 2. Creates a new Amethyst game with all the appropriate bundles;
/// 3. Sets out the rendering passes: background, sprite drawing and UI;
/// 4. Sets the game running. Control is now passed to the game state.
pub fn run() -> Result<(), amethyst::Error> {
    let _ = &config::GAME_CONFIGURATION; // initialises game constants

    let display_config_path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let display_config = DisplayConfig::load(&display_config_path);

    let key_bindings_path = format!(
        "{}/resources/input.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let resources_path = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));

    let game = Application::build(
        resources_path,
        GameState)?
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(
                &key_bindings_path
            ),
        )?
        .with_bundle(RenderBundle::new())?
        .with_bundle(UiBundle::new())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(GameBundle)?;

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

/// Main method
///
/// Let [run](run.v.html) do all the work, and just print out any error it generates.
pub fn main() {
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}
