

use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::{PosTex, MeshHandle, Material};
use amethyst::assets::{Loader};
use amethyst::core::transform::{LocalTransform, Transform};

// CONSTANTS WHICH DRIVE OUR SYSTEM:
const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;

const BALL_RADIUS: f32 = 2.5;
const BALL_COLOUR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

/// Initialises one ball in the middle-ish of the arena.
pub fn initialise_ball(world: &mut World) -> Entity {

    // Create the mesh, material and translation.
    let mesh = create_mesh(world, generate_circle_vertices(BALL_RADIUS, 16));
    let material = create_colour_material(world, BALL_COLOUR);
    let mut local_transform = LocalTransform::default();
    local_transform.translation = Vector3::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(local_transform)
        .with(Transform::default())
        .build()
}

/// Generates vertices for a circle. The circle will be made of `resolution`
/// triangles.
fn generate_circle_vertices(radius: f32, resolution: usize) -> Vec<PosTex> {
    use std::f32::consts::PI;

    let mut vertices = Vec::with_capacity(resolution * 3);
    let angle_offset = 2.0 * PI / resolution as f32;

    // Helper function to generate the vertex at the specified angle.
    let generate_vertex = |angle: f32| {
        let x = angle.cos();
        let y = angle.sin();
        PosTex {
            position: [x * radius, y * radius, 0.0],
            tex_coord: [x, y],
        }
    };

    for index in 0..resolution {
        vertices.push(PosTex {
            position: [0.0, 0.0, 0.0],
            tex_coord: [0.0, 0.0],
        });

        vertices.push(generate_vertex(angle_offset * index as f32));
        vertices.push(generate_vertex(angle_offset * (index + 1) as f32));
    }

    vertices
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
