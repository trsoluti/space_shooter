
use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::{PosTex, MeshHandle, Material};
use amethyst::assets::{Loader};
use amethyst::core::transform::{LocalTransform, Transform};

use super::background::create_png_vertices;

// CONSTANTS WHICH DRIVE OUR SYSTEM:
const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;

const PADDLE_HEIGHT: f32 = 15.0;
const PADDLE_WIDTH: f32 = 2.5;
const PADDLE_COLOUR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

/// Initialises one ball in the middle-ish of the arena.
pub fn initialise_paddle(world: &mut World) -> Entity {

    let mut left_transform = LocalTransform::default();
    let mut right_transform = LocalTransform::default();

    // Correctly position the paddles.
    let y = (ARENA_HEIGHT - PADDLE_HEIGHT) / 2.0;
    left_transform.translation = Vector3::new(0.0, y, 0.0);
    right_transform.translation = Vector3::new(ARENA_WIDTH - PADDLE_WIDTH, y, 0.0);

    // Create the mesh and the material needed.
    let mesh = create_mesh(
        world,
        create_png_vertices(0.0, 0.0, PADDLE_WIDTH, PADDLE_HEIGHT),
    );

    let material = create_colour_material(world, PADDLE_COLOUR);

    // Create a left plank entity.
    world
        .create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(Transform::default())
        .with(left_transform)
        .build()
}

/// Converts a vector of vertices into a mesh.
fn create_mesh(world: &World, vertices: Vec<PosTex>) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

/// Creates a solid material of the specified colour.
fn create_colour_material(world: &World, colour: [f32; 4]) -> Material {
    // TODO: optimize

    use amethyst::renderer::MaterialDefaults;

    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(colour.into(), (), &world.read_resource());

    Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}
