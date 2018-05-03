//! Manage all the asteroid entities
//!
//! This module implements a re-use pattern to manage the asteroids.
//!
//! The initialisation method creates a set of a hundred asteroids
//! that will be used over and over in a game.
//!
//! When an asteroid falls out of camera range or is hit by a ship
//! or a laser, the appropriate system will relocate it to back above
//! the screen using the [locate_asteroid](fn.locate_asteroid.html)
//! function.
//!
//! This type of pattern is useful when you want the appearance of
//! an infinite number of objects but you don't want the overhead
//! of creating and destroying them all the time.
//!
//! (Note the underlying world actually maintains a list of deleted
//! (destroyed) entities and re-uses the slots when you create new ones,
//! so both methods have the same underlying implementation.)

use amethyst::ecs::prelude::{Entity, World};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::core::cgmath::Vector3;
use rand::{Rng, ThreadRng, thread_rng};

use super::png_mesh_and_material;
use config::GAME_CONFIGURATION;
use components::Asteroid;

/// Initialises a hundred asteroid objects somewhere above the arena.
///
/// The method first sets up the resources each entity will need,
/// then uses a random number generator and a location function
/// to position the asteroids above the screen top.
///
/// This implements a pattern of creating a fixed number of entities
/// and re-using them to make them appear endless. It is less wasteful
/// than creating and destroying entities on the fly (like [lasers](../laser/index.html)).
///
/// The asteroid entities are returned as a vector, in case
/// we want to use them somewhere else.
pub fn initialise_asteroids(world: &mut World) -> Vec<Entity> {
    let (mesh, background) = png_mesh_and_material("PNG/Meteors/meteorBrown_med1.png", [43.0,43.0], world);
    let asteroid = Asteroid {
        velocity: GAME_CONFIGURATION.asteroid_velocity,
        width: 43.0,
        height: 43.0,
        is_destroyed: false,
    };
    let (screen_width, screen_height) = {
        (1024.,1024.)
    };

    let mut rng = thread_rng();

    let numbers = 0..;
    let range = numbers.take(100);
    range.map(|_number| {
        let local_transform = locate_asteroid(&asteroid, screen_width, screen_height, &mut rng);

        world
        .create_entity()
        .with(mesh.clone())
        .with(background.clone())
        .with(asteroid.clone())
        .with(local_transform)
        .with(GlobalTransform::default())
        .build()
    }).collect()
}

/// (Re)locate the asteroid to a random spot somewhere above the screen.
///
/// The speed at which the asteroids fall is calculated into the
/// placement, so the player has time to get used to the game
/// before having to fire asteroids.
///
/// As well, the height of the field is the same as the screen size,
/// so the asteroids will fall at continuously regular intervals.
pub fn locate_asteroid(asteroid: &Asteroid, screen_width: f32, screen_height: f32, random_number_generator: &mut ThreadRng)->Transform {
    let max_width = screen_width - asteroid.width;
    let min_height = screen_height + GAME_CONFIGURATION.wait_for_first_asteroid * GAME_CONFIGURATION.asteroid_velocity;
    let max_height = min_height +  (screen_height * GAME_CONFIGURATION.asteroid_velocity) / GAME_CONFIGURATION.asteroid_density;
    let pos_x = random_number_generator.next_f32()*max_width;
    let pos_y = min_height + random_number_generator.next_f32()*(max_height-min_height);

    let mut local_transform = Transform::default();
    local_transform.translation = Vector3::new(pos_x, pos_y, 0.0);
    local_transform
}
