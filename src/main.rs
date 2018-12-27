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
//!     <td><a href="resources/index.html">resources</a></td>
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
//!     <td>The collection of systems that make up the game.</td>
//!   </tr>
//!   <tr>
//!     <td><a href="struct.GameConfiguration.html">game configuration</td>
//!     <td>The set of items used by the Game Designer to tune the game after the coding is complete (e.g. player speed)</td>
//!   </tr>
//! </table>
//!
//! These items are put in their own Rust files.

#![deny(missing_docs)]

//extern crate amethyst;
//extern crate serde;
//#[macro_use]
//extern crate serde_derive;
//#[macro_use]
//extern crate lazy_static;
//extern crate rand;
// These next two crates are needed only for the arguments to with_transparency()
// eventually this will be blended into Amethyst somehow.
//extern crate gfx;
//extern crate gfx_core;

mod config;
mod state;
mod bundle;
pub mod components;
pub mod entities;
pub mod systems;
pub mod resources;

// public use so these things get documented
pub use crate::config::GameConfiguration;
pub use crate::config::GAME_CONFIGURATION;
pub use crate::state::GameState;
pub use crate::bundle::GameBundle;

use amethyst::prelude::*;
use amethyst::utils::application_root_dir;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DisplayConfig, RenderBundle, Pipeline, Stage, DrawFlat, PosTex};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::input::InputBundle;
// These next two values are needed only for the arguments to with_transparency()
// eventually this will be blended into Amethyst somehow.
use gfx_core::state::{ColorMask};
use gfx::preset::blend::ALPHA;



const BACKGROUND_COLOUR: [f32; 4] = [0.25, 0.25, 0.25, 0.0]; // dark grey

/// Run the game
///
/// This function does the following:
///
/// 1. Loads up the display configuration and input bindings from RON files
///    in the resources folder;
/// 2. Sets out the rendering pipeline: background rendering pass, sprite rendering pass and UI rendering pass;
/// 3. Creates a new Amethyst game data object with all the appropriate bundles;
/// 4. Creates a new Amethyst game with the game data and our [GameState].
/// 5. Sets the game running. Control is now passed to the game state.
pub fn run() -> Result<(), amethyst::Error> {
    let _ = &config::GAME_CONFIGURATION; // initialises game constants

    let application_root = application_root_dir()?;

    // Set the display configuration path to <package root>/resources/display_config.ron.
    let display_config_path =  application_root.join("resources/display_config.ron");
    let display_config = DisplayConfig::load(&display_config_path);

    // Load up the key bindings path and the resources path
    let key_bindings_path = application_root.join("resources/input.ron");

    let resources_path = application_root.join("assets");

    // Create a pipeline that has three passes:
    // 1. setting the background to the background colour constant
    // 2. drawing the background image and the sprites
    // 3. drawing the UI components (lives)
    let pipe = {
        Pipeline::build()
            .with_stage(
                Stage::with_backbuffer()
                    .clear_target(BACKGROUND_COLOUR, 1.0)
                    .with_pass(DrawFlat::<PosTex>::new().with_transparency(ColorMask::all(), ALPHA, None))
                    .with_pass(DrawUi::new())
            )

    };

    // Create a game data with all our systems bundled into it
    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(
                &key_bindings_path
            )?,
        )?
        .with_bundle(GameBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?;

    // Create a game with out game data and our GameState.
    let mut game = Application::new(
        resources_path,
        GameState,
        game_data)?;

    Ok(
        game.run(),
    )
}

/// Main method
///
/// Let [run](run.v.html) do all the work, and just print out any error it generates.
pub fn main() {
    amethyst::start_logger(Default::default());
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}
