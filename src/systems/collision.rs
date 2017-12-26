
use amethyst::core::transform::LocalTransform;
use amethyst::ecs::{Join, System, ReadStorage, WriteStorage, FetchMut};

use components::Ship;
use components::Asteroid;
use resources::PlayState;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        FetchMut<'s, PlayState>,
        WriteStorage<'s, Ship>,
        ReadStorage<'s, LocalTransform>,
        WriteStorage<'s, Asteroid>,
    );

    fn run(&mut self, (mut play_state, mut ships, transforms, mut asteroids): Self::SystemData) {
        for (ship_component, ship_transform) in (&mut ships, &transforms).join() {
            let ship_left = ship_transform.translation[0];
            let ship_top = ship_transform.translation[1] + ship_component.height;
            let ship_right = ship_left + ship_component.width;

            // Check to see if our ship has collided with any asteroid
            for (asteroid_component, asteroid_transform) in (&mut asteroids, &transforms).join() {
                let asteroid_left = asteroid_transform.translation[0];
                let asteroid_bottom = asteroid_transform.translation[1];
                let asteroid_right = asteroid_left + asteroid_component.width;

                if ((ship_left <= asteroid_right && ship_left >= asteroid_left)
                    || (ship_right <= asteroid_left && ship_right >= asteroid_right))
                    && (ship_top >= asteroid_bottom) {
                    // we have a collision. Decrement the number of lives of the game
                    play_state.lives -= 1;
                    // let the respawn system know the asteroid is destroyed and ready for respawn
                    asteroid_component.is_destroyed = true;
                }
            }
        }
    }
}