//! Manage the laser entities
//!
//! This module uses a create-destroy pattern to manage the entities.
//!
//! The laser template is created as a resource that a system can access.
//!
//! The ship system then creates a laser on the fly,
//! using a reference to that resource,
//! when the user presses the FIRE button.
//!
//! The laser movement system will destroy the laser when it runs out of
//! camera range or hits an asteroid.
use amethyst::ecs::prelude::{World, Entities, Entity, LazyUpdate, ReadExpect};
use amethyst::core::transform::{Transform};
use amethyst::core::nalgebra::Vector3;

use super::png_mesh_and_material;
use crate::config::GAME_CONFIGURATION;
use crate::components::Laser as LaserComponent;
use crate::resources::LaserResource;

/// Initialises the data we use to instantiate a laser when fired.
///
/// This function creates a mesh, a material and a component
/// that will be attached to the entity when we create it in
/// [fire_laser](fn.fire_laser.html).
pub fn initialise_laser_resource(world: &mut World) -> LaserResource {
    let (mesh, material) = png_mesh_and_material("PNG/Lasers/LaserRed01.png", [9.0,54.0], world);
    let laser_resource = LaserResource {
        mesh,
        material,
        component: LaserComponent {
            velocity: GAME_CONFIGURATION.laser_velocity,
            width: 9.0,
            height: 54.0,
        },
    };
    world.add_resource(laser_resource.clone());
    laser_resource
}

/// Fires the laser at the given position.
///
/// This is a pattern for instantiating an entity from
/// within a System. We use a lazy create on our list of
/// entities to queue our create requests.
///
/// When the Amethyst engine calls world.maintain(),
/// it will create the laser entity.
pub fn fire_laser(
    entities: &Entities,
    laser_resource: &ReadExpect<LaserResource>,
    fire_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>)
{
    let laser_entity:Entity = entities.create();
    let local_transform = {
        let mut local_transform = Transform::default();
        local_transform.set_position(fire_position);
        // the fire position actually represents the middle of our laser. Adjust accordingly.
        let p = local_transform.translation()[0];
        local_transform.set_x(p - laser_resource.component.width / 2.0);
        local_transform
    };
    lazy_update.insert(laser_entity, laser_resource.material.clone());
    lazy_update.insert(laser_entity, laser_resource.mesh.clone());
    lazy_update.insert(laser_entity, laser_resource.component.clone());
    lazy_update.insert(laser_entity, local_transform);
}