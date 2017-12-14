mod background;
mod camera;
mod ship;
mod asteroid;

use amethyst::ecs::World;
use amethyst::renderer::{PngFormat, Texture, PosTex, Mesh, Material, MaterialDefaults};
use amethyst::assets::{AssetStorage, Loader, Handle};

pub fn initialise_entities(world: &mut World) {
    background::initialise_background(world);
    ship::initialise_ship(world);
    asteroid::initialise_asteroids(world);
    camera::initialise_camera(world);
}

pub fn png_mesh_and_material(name: &'static str, png_size: [f32; 2], world: &mut World)-> (Handle<Mesh>, Material) {
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load(
        name,
        PngFormat,
        Default::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    );

    let mat_defaults = world.read_resource::<MaterialDefaults>();

    let material = Material {
        albedo,
        ..mat_defaults.0.clone()
    };

    let vertices = create_png_vertices(0.0, 0.0,png_size[0],png_size[1]);

    let mesh = loader.load_from_data(
        vertices.into(),
        (),
        &world.read_resource::<AssetStorage<Mesh>>());
    (mesh, material)
}

fn create_png_vertices(left: f32, bottom: f32, right:f32, top:f32) -> Vec<PosTex> {
    vec![
        PosTex {
            position: [left, bottom, 0.0],
            tex_coord: [0.0, 0.0],
        },
        PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [1.0, 0.0],
        },
        PosTex {
            position: [left, top, 0.0],
            tex_coord: [0.0, 1.0],
        },
        PosTex {
            position: [right, top, 0.0],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [left, top, 0.],
            tex_coord: [0.0, 1.0],
        },
        PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [1.0, 0.0],
        },
    ]
}
