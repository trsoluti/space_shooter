use components::Ship;
use resources::LaserResource;
use entities::fire_laser;
use config::GAME_CONFIGURATION;

use amethyst::core::cgmath::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{ReadExpect, Join, System, WriteStorage, Entities, LazyUpdate};
use amethyst::input::InputHandler;

/// Moves the ship and fires lasers based on user-provided input.
///
/// This system implements a pattern of thrust- (or force-) based movement.
/// When the player moves the joystick along an axis, that applies thrust
/// to the ship's velocity.
///
/// The ship also bounces off either side of the screen as if they were walls.
///
/// This is also an example of keeping a list of entities when we know
/// from a design point of view we have only one. It needs to be an entity
/// so it can be rendered, and entities always come in lists, even if the
/// list only has one entry.
pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    /// The data for each pass of the ship system
    /// We need:
    ///
    /// * **Entities**:          the list of entities so we can add a fired laser to them
    /// * **Ships**:             write access to the list of ship(s)
    ///                            so we can update the ship's velocity
    /// * **Transforms**:        write access to the list of ship position(s)
    ///                            so we can update the ship's location
    /// * **Time**:              read access to the time resource so we can know how much time
    ///                            has elapsed since we last ran this system
    /// * **Input Handler**:     read access to the input handler so we can
    ///                            sample the position of the "joystick" and the fire "button"
    /// * **LaserResource**:     read access to the laser creation resources we set up in
    ///                            <a href="../entities/laser/fn.initialise_laser_resource.html">initialise_laser</a>
    /// * **Lazy Update**:       a mechanism that queues changes to the world
    ///                            until after all the systems have run. We use this
    ///                            to create a laser entity with all its related components.
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, Time>,
        ReadExpect<'s, InputHandler<String, String>>,
        ReadExpect<'s, LaserResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    /// Runs a pass of the system on our selected components
    ///
    /// This function is given a list of the components described under [SystemData](#associatedtype.SystemData).
    /// It does an operation for each ship with its related position (transform).
    /// <em>(We don't care that there's only one ship; this is the appropriate way to
    /// extract the ship and transport component from our storage.)</em>
    ///
    /// We can then figure out whether or not we can fire the laser,
    /// then whether or not the user wants to fire the laser,
    /// then whether or not the user wants to apply thrust to our ship.
    ///
    /// If the user wants to fire and we have a laser ready, we
    /// call the [fire_laser](../entities/laser/fn.fire_laser.html) function
    /// with the correct resources so it can queue a request to create our laser entity.
    ///
    /// If the user moved the "joystick" off-centre (e.g. pressed **a** or **d** keys),
    /// apply the appropriate thrust to the ship's velocity.
    ///
    /// Finally, we move the ship and bounce it off the bounds if we have hit them.
    fn run(&mut self, (entities, mut ships, mut transforms, time, input, laser_resource, lazy_update): Self::SystemData) {
        for (ship, transform) in (&mut ships, &mut transforms).join() {
            // count down on the amount of time before we can fire again.
            if ship.trigger_reset_timer > 0.0 {
                ship.trigger_reset_timer -= time.delta_seconds();
            }
            // get the current 'joystick' reading and whether the fire button has been pressed.
            let optional_movement = input.axis_value("ship");
            let optional_action = input.action_is_down("fire");

            // if the fire button is down,
            if let Some(action) = optional_action {
                // and sufficient time has passed since we last fired,
                if action && ship.trigger_reset_timer <= 0.0 {
                    // fire from the middle top of the ship.
                    let fire_position = Vector3{
                        x: transform.translation[0] + ship.width / 2.0,
                        y: transform.translation[1] + ship.height,
                        z: 0.0,
                    };
                    fire_laser(&entities, &laser_resource, fire_position, &lazy_update);

                    // reset the timer so we can't fire again until the timeout has elapsed.
                    ship.trigger_reset_timer = GAME_CONFIGURATION.trigger_reset_timeout;
                }
            }

            // if joystick is off centre,
            if let Some(movement) = optional_movement {
                ship.velocity += movement as f32 * time.delta_seconds() * GAME_CONFIGURATION.ship_thrust;
            }

            // move the ship according to its velocity
            transform.translation[0] -= ship.velocity * time.delta_seconds();

            // make sure the ship stays on the screen
            let arena_width = 1024.;
            let max_position = arena_width - (ship.width/2.0);
            if transform.translation[0] <= (-ship.width/2.0) {
                transform.translation[0] = -ship.width/2.0;
                ship.velocity = -ship.velocity; // bounce off the left wall
            } else if transform.translation[0] >= max_position {
                transform.translation[0] = max_position;
                ship.velocity = - ship.velocity; // bounce off right wall
            }
        }
    }
}

