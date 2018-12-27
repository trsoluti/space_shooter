
use amethyst::ecs::prelude::{Read, Join, System, ReadStorage, Entities};

use crate::components::Life;
use crate::resources::PlayState;

/// Manages the list of life icons in the UI layer based on the play state
///
/// This is a good pattern for managing icons that represent number of lives.
///
/// It updates a list of entities based on a game-wide resource.
pub struct LivesSystem;

impl<'s> System<'s> for LivesSystem {
    /// The data for each pass of the lives system
    /// We need:
    ///
    /// * **Entities**:  the list of entities so we can delete a life icon
    ///                    when it's no longer valid
    /// * **Lives**:     read access to the list of life components
    ///                    so we can check their life number
    /// * **PlayState**: read access to the play state
    ///                    so we can read the current number of lives
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Life>,
        Read<'s, PlayState>,
    );

    /// Runs a pass of the system on our selected components
    ///
    /// This function is given a list of the components described under [SystemData](#associatedtype.SystemData).
    ///
    /// It selects the life entities and components, and, for each life,
    /// checks to see if its number is greater than the current number
    /// of player lives in the game.
    /// If it is, the function asks the entity list to delete the life entity.
    ///
    /// Note that we have no need of the life transform, since we don't move the life.
    fn run(&mut self, (entities, lives, play_state): Self::SystemData) {
        for (life_entity, life_component) in (&*entities, &lives).join() {
            if life_component.life_number >= play_state.lives {
                let _result = entities.delete(life_entity);
            }
        }
    }

}