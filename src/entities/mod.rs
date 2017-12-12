mod background;
mod camera;
mod ball;
mod paddle;

use amethyst::ecs::World;

pub fn initialise_entities(world: &mut World) {
    background::initialise_background(world);
    ball::initialise_ball(world);
    paddle::initialise_paddle(world);
    camera::initialise_camera(world);
}
