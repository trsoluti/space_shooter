
use amethyst::ecs::{Entity, World};
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::{PngFormat, Texture, PosTex, Mesh, Material, MaterialDefaults};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::{LocalTransform, Transform};

pub fn initialise_background(world: &mut World) -> Entity {
    let (mesh, background) = {
        let loader = world.read_resource::<Loader>();

        let background_texture = loader.load(
            "texture/background.png",
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

        let vertices = create_png_vertices(0.0, 0.0,1300.0,1024.0);

        let mesh = loader.load_from_data(
            vertices.into(),
            (),
            &world.read_resource::<AssetStorage<Mesh>>());
        (mesh, background)
    };

    let mut local_transform = LocalTransform::default();
    local_transform.translation = Vector3::new(0.0, 0.0, 0.0);
    local_transform.scale = Vector3::new(0.1, 0.1, 1.0);

    world
        .create_entity()
        .with(mesh)
        .with(background)
        .with(local_transform)
        .with(Transform::default())
        .build()
}

pub fn create_png_vertices(left: f32, bottom: f32, right:f32, top:f32) -> Vec<PosTex> {
    vec![
        PosTex {
            position: [left, bottom, 0.],
            tex_coord: [0.0, 0.0],
        },
        PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [1.0, 0.0],
        },
        PosTex {
            position: [left, top, 0.0],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [right, top, 0.],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [left, top, 0.],
            tex_coord: [0.0, 1.0],
        },
        PosTex {
            position: [right, bottom, 0.0],
            tex_coord: [0.0, 0.0],
        },
    ]
}
