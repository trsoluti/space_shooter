
use amethyst::ecs::{World, Entities, Entity, LazyUpdate, Fetch};
use amethyst::core::transform::{Transform, LocalTransform};
use amethyst::core::cgmath::Vector3;

use super::png_mesh_and_material;
use config::GAME_CONFIGURATION;
use components::Laser as LaserComponent;
use resources::LaserResource;

/// Initialises the data we use to lo
pub fn initialise_laser_resource(world: &mut World) -> LaserResource {
    let (mesh, material) = png_mesh_and_material("PNG/Lasers/LaserRed01.png", [9.0,54.0], world);
    let laser_resource = LaserResource {
        mesh,
        material,
        component: LaserComponent {
            velocity: GAME_CONFIGURATION.laser_velocity,
            width: 9.0 * 0.1,
            height: 54.0 * 0.1,
        },
    };
    world.add_resource(laser_resource.clone());
    laser_resource
}

pub fn fire_laser(entities: &Entities, laser_resource: &Fetch<LaserResource>, fire_position: Vector3<f32>, lazy_update: &Fetch<LazyUpdate>) {
    let laser_entity:Entity = entities.create();
    let local_transform = {
        let mut local_transform = LocalTransform::default();
        local_transform.translation = fire_position;
        // the fire position actually represents the middle of our laser. Adjust accordingly.
        local_transform.translation[0] -= laser_resource.component.width / 2.0;
        // scale the item properly:
        local_transform.scale = Vector3::new(0.1, 0.1, 1.0);
        local_transform
    };
    lazy_update.insert(laser_entity, laser_resource.material.clone());
    lazy_update.insert(laser_entity, laser_resource.mesh.clone());
    lazy_update.insert(laser_entity, laser_resource.component.clone());
    lazy_update.insert(laser_entity, local_transform);
    lazy_update.insert(laser_entity, Transform::default());
}