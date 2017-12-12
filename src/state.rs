
use amethyst::prelude::*;
use amethyst::ecs::{World};
use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};

use entities::initialise_entities;

pub struct SpriteState;


impl State for SpriteState {
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
}