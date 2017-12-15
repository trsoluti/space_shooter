
use amethyst::ecs::{Entity, World};
use amethyst::core::transform::{Transform};
use amethyst::renderer::ScreenDimensions;
use rand::thread_rng;

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
    let (screen_width, screen_height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut rng = thread_rng();

    let numbers = 0..;
    let range = numbers.take(100);
    range.map(|_number| {
        let local_transform = asteroid.locate(screen_width, screen_height, &mut rng);

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
