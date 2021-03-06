//! These are all the entities that the game uses.
//!
//! Entities are not rust structures, but a registration of a set of related
//! components within the game world.
//!
//! The entities used by the space_shooter game are:
//!
//! * **background** - a simple (untiled) background of stars
//! * **camera**     - a camera that encapsulates the scene
//! * **ship**       - the player's ship, which responds to keypresses
//! * **asteroid**   - the asteroids which collide with the ship (a fixed number, which are re-used)
//! * **laser**      - the bullets the ship uses to fire on the asteroids (created and destroyed on demand)
//! * **lives**      - the ships on the UI layer that represent the number of remaining lives.
//!
//! Note each initialisation method returns the entity (or list of entities, or resource) it creates,
//! in case you wanted to create entities that are related to other entities. That's just good practice.
pub mod asteroid;
pub mod background;
pub mod camera;
pub mod laser;
pub mod lives;
pub mod ship;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::prelude::World;
use amethyst::ecs::prelude::WorldExt;
use amethyst::renderer::formats::texture::ImageFormat;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::Texture;

pub use self::asteroid::locate_asteroid;
pub use self::laser::fire_laser;

/// Initialises all the entities (some are just set up as resources so the entities can be created later on demand)
pub fn initialise_entities(world: &mut World) {
    // Load the sprite sheet with all our entities
    let sprite_sheet_handle = {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "Spritesheet/sheet.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "Spritesheet/sheet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        )
    };
    background::initialise_background(world);
    ship::initialise_ship(world, sprite_sheet_handle.clone());
    asteroid::initialise_asteroids(world, sprite_sheet_handle.clone());
    camera::initialise_camera(world);
    laser::initialise_laser_resource(world, sprite_sheet_handle.clone());
    lives::initialise_lives(world, sprite_sheet_handle.clone());
}

///// Loads a material (png file) and creates a mesh (display object) that is the same size as the material,
///// which is what sprites are.
/////
///// This code may end up being moved inside Amethyst, where it can get the size
///// from the png file itself.
//pub fn png_mesh_and_material(name: &'static str, png_size: [f32; 2], world: &mut World)-> (Handle<Mesh>, Handle<Material>) {
//    let loader = world.read_resource::<Loader>();
//
//    let albedo = loader.load(
//        name,
//        ImageFormat::default(),
//        (),
//        &world.read_resource::<AssetStorage<Texture>>(),
//    );
//
//    let mat_defaults = world.read_resource::<MaterialDefaults>();
//
//    let material = Material {
//        albedo,
//        ..mat_defaults.0.clone()
//    };
//
//    let material_handlers = &world.read_resource::<AssetStorage<Material>>();
//    let material_handle = loader.load_from_data(material, (),material_handlers);
//
//    let vertices = create_png_vertices(0.0, 0.0,png_size[0],png_size[1]);
//    let mesh_builder = MeshBuilder::new().with_vertices(vertices);
//
//    let mesh = loader.load_from_data(
//        /*vertices.into()*/mesh_builder.into(),
//        (),
//        &world.read_resource::<AssetStorage<Mesh>>());
//
//    (mesh, material_handle)
//}
//
///// Creates a list of vertices which make a simple plane of the given dimensions.
/////
///// The vertices can then be loaded into the world as a mesh.
//pub fn create_png_vertices(left: f32, bottom: f32, right:f32, top:f32) -> Vec<PosTex> {
//    vec![
//        PosTex {
//            position: [left, bottom, 0.].into(),
//            tex_coord: [0.0, 0.0].into(),
//        },
//        PosTex {
//            position: [right, bottom, 0.].into(),
//            tex_coord: [1.0, 0.0].into(),
//        },
//        PosTex {
//            position: [left, top, 0.].into(),
//            tex_coord: [0.0, 1.0].into(),
//        },
//        PosTex {
//            position: [right, top, 0.].into(),
//            tex_coord: [1.0, 1.0].into(),
//        },
//        PosTex {
//            position: [left, top, 0.].into(),
//            tex_coord: [0.0, 1.0].into(),
//        },
//        PosTex {
//            position: [right, bottom, 0.0].into(),
//            tex_coord: [1.0, 0.0].into(),
//        },
//    ]
//}
