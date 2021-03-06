//use amethyst::renderer::{Material, Mesh};
use crate::components::Laser as LaserComponent;
use amethyst::renderer::SpriteRender;

/// The resource containing data we need to create a laser entity.
///
/// This saves time as the system doesn't need to load the sprite
/// from the resource folder every time the player fires a laser.
///
/// The resource is marked "Cloneable" so it can be used over and over
/// to create new entities.
#[derive(Clone)]
pub struct LaserResource {
    /// The component used to create a laser entity
    pub component: LaserComponent,
    /// The render that locates the sprite in a sprite sheet resource
    pub sprite_render: SpriteRender,
}
