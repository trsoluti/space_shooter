mod background;
mod camera;
mod ship;
mod paddle;

use amethyst::ecs::World;

pub fn initialise_entities(world: &mut World) {
    background::initialise_background(world);
    ship::initialise_ship(world);
    paddle::initialise_paddle(world);
    camera::initialise_camera(world);
}
