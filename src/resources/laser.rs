
use amethyst::renderer::{Material, Mesh};
use amethyst::assets::Handle;
use crate::components::Laser as LaserComponent;

/// The resource containing data we need to create a laser entity.
///
/// This saves time as the system doesn't need to load the sprite
/// from the resource folder every time the player fires a laser.
///
/// The resource is marked "Cloneable" so it can be used over and over
/// to create new entities.
#[derive(Clone)]
pub struct LaserResource {
    /// The mesh used to create a laser entity
    pub mesh: Handle<Mesh>,
    /// The material used to create a laser entity
    pub material: Material,
    /// The component used to create a laser entity
    pub component: LaserComponent,
}
