
use amethyst::prelude::*;
use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};

use entities::initialise_entities;
use resources::add_resources;
use components::register_components;
use resources::{PlayState};

/// The rules on what to do at each point of the game
///
/// This game state demonstrates several standard patterns:
///
/// 1. Initialising all the entities on start
/// 2. Handling the Escape key to stop the game
/// 3. Monitoring a resource to determine the end of a game
/// (and passing information from the system to the game state through a resource).
pub struct GameState;


impl<'a, 'b> State<GameData<'a, 'b>> for GameState {
    fn on_start(&mut self, state_data: StateData<GameData>) {
        let world = state_data.world;
        register_components(world);
        add_resources(world);
        initialise_entities(world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    // Stop the game if the ship runs out of lives
    fn fixed_update(&mut self, state_data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        let world = state_data.world;
        let play_state = world.read_resource::<PlayState>();
        if play_state.lives == 0 { Trans::Quit} else { Trans::None }
    }

    // This code tells Amethyst to run all the systems in your game data.
    fn update (&mut self, state_data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        state_data.data.update(&state_data.world);
        Trans::None
    }
}