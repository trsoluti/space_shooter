

use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::{PngFormat, Texture, PosTex, Mesh, Material, MaterialDefaults};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::{LocalTransform, Transform};

use super::background::create_png_vertices;

// CONSTANTS WHICH DRIVE OUR SYSTEM:
//const ARENA_HEIGHT: f32 = 100.0;
//const ARENA_WIDTH: f32 = 100.0;

//const BALL_RADIUS: f32 = 2.5;
//const BALL_COLOUR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

/// Initialises one ball in the middle-ish of the arena.
pub fn initialise_ship(world: &mut World) -> Entity {
    let (mesh, background) = {
        let loader = world.read_resource::<Loader>();

        let background_texture = loader.load(
            "texture/spaceship.png",
            PngFormat,
            Default::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        );

        let mat_defaults = world.read_resource::<MaterialDefaults>();

        let background = Material {
            albedo: background_texture,
            ..mat_defaults.0.clone()
        };

        let vertices = create_png_vertices(0.0, 0.0,103.0,84.0);

        let mesh = loader.load_from_data(
            vertices.into(),
            (),
            &world.read_resource::<AssetStorage<Mesh>>());
        (mesh, background)
    };

    let mut local_transform = LocalTransform::default();
    local_transform.translation = Vector3::new(10.0, 10.0, 0.0);
    local_transform.scale = Vector3::new(0.1, 0.1, 1.0);

    world
        .create_entity()
        .with(mesh)
        .with(background)
        .with(local_transform)
        .with(Transform::default())
        .build()
}
