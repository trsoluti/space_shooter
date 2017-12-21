
use amethyst::renderer::{Material, Mesh};
use amethyst::assets::Handle;
use components::Laser as LaserComponent;

#[derive(Clone)]
pub struct LaserResource {
    pub mesh: Handle<Mesh>,
    pub material: Material,
    pub component: LaserComponent,
}