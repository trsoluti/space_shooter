
use amethyst::ecs::{Fetch, Join, System, ReadStorage, Entities};

use components::Life;
use resources::PlayState;

pub struct LivesSystem;

impl<'s> System<'s> for LivesSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Life>,
        Fetch<'s, PlayState>,
    );

    fn run(&mut self, (entities, lives, play_state): Self::SystemData) {
        for (life_entity, life_component) in (&*entities, &lives).join() {
            if life_component.life_number >= play_state.lives {
                let _result = entities.delete(life_entity);
            }
        }
    }

}