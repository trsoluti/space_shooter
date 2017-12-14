
use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{LocalTransform, Transform};
use amethyst::renderer::ScreenDimensions;

use super::png_mesh_and_material;
use config::GAME_CONFIGURATION;
use components::Asteroid;

/// Initialises a hundred asteroid objects somewhere above the arena.
pub fn initialise_asteroids(world: &mut World) -> Vec<Entity> {
    let (mesh, background) = png_mesh_and_material("PNG/Meteors/meteorBrown_med1.png", [43.0,43.0], world);
    let asteroid = Asteroid {
        velocity: GAME_CONFIGURATION.ship_velocity,
        width: 43.0*0.1,
        height: 43.0 * 0.1,
        is_destroyed: false,
    };
    let screen_height = {
        let dim = world.read_resource::<ScreenDimensions>();
        dim.height()
    };

    let numbers = 0..;
    let range = numbers.take(100);
    range.map(|number| {
        let mut local_transform = LocalTransform::default();
        local_transform.translation = Vector3::new(number as f32, screen_height + ((number % 12) as f32) * asteroid.height, 0.0);
        local_transform.scale = Vector3::new(0.1, 0.1, 1.0);

        world
        .create_entity()
        .with(mesh.clone())
        .with(background.clone())
        .with(asteroid.clone())
        .with(local_transform)
        .with(Transform::default())
        .build()
    }).collect()


}
