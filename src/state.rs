
use amethyst::prelude::*;
use amethyst::ecs::{World};
use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};

use entities::initialise_entities;
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


impl State for GameState {
    fn on_start(&mut self, world: &mut World) {
        initialise_entities(world);
    }

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
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
    fn fixed_update(&mut self, world: &mut World) -> Trans {
        let play_state = world.read_resource::<PlayState>();
        if play_state.lives == 0 { Trans::Quit} else { Trans::None }
    }
}