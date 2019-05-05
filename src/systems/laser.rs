
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entities, Join, System, ReadStorage, WriteStorage, Read};
use amethyst::core::timing::Time;

use crate::components::Laser;
use crate::config::GAME_CONFIGURATION;

/// Moves the laser and deletes it if it goes off the screen
///
/// This is a good pattern of one of the simplest systems:
/// one that handles movement and destruction on out-of-bounds.
pub struct LaserSystem;

impl<'s> System<'s> for LaserSystem {
    /// The data for each pass of the laser system
    /// We need:
    ///
    /// * **Entities**:          the list of entities so we can delete the laser
    ///                            when it goes out of bounds
    /// * **Lasers**:            read access to the list of laser components
    ///                            so we select only the laser entities and transforms
    /// * **Transforms**:        write access to the list of transforms
    ///                            so we can update the laser positions
    /// * **Time**:              read access to the time resource so we can know how much time
    ///                            has elapsed since we last ran this system
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Laser>,
        WriteStorage<'s, Transform<f32>>,
        Read<'s, Time>,
    );

    /// Runs a pass of the system on our selected components
    ///
    /// This function is given a list of the components described under [SystemData](#associatedtype.SystemData).
    ///
    /// This is a good example of how entity pattern matching works.
    /// We don't use anything in the laser component (which is why it is named `_laser_component`).
    /// It is only in our list so that, when we do the join,
    /// we pick out only the transforms and entities that have a laser component.
    ///
    /// This function scans the list of entities, lasers and transforms for laser entities and transforms.
    ///
    /// For each laser transform, it updates the position based on the laser velocity in our game configuration.
    ///
    /// The function then checks the laser's position against the screen top. If the laser has gone off the screen,
    /// it asks the entity list to queue a request to delete the selected laser entity.
    /// (The deletion will happen after all the systems have run and the Amethyst engine does a `world.maintain()`.)
    fn run(&mut self, (entities, lasers, mut transforms, time): Self::SystemData) {
        // Scan through the list of lasers and move them forward.
        for (laser_entity, _laser_component, laser_transform) in (&*entities, &lasers, &mut transforms).join() {
            laser_transform.prepend_translation_y(GAME_CONFIGURATION.laser_velocity * time.delta_seconds());

            // Delete the laser if it has gone off the screen
            if laser_transform.translation()[1] > 1024. {
                let _result = entities.delete(laser_entity);
            }
        }
    }
}