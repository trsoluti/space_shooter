//! These are all the systems that implement the rules and procedures of the game.
//!
//! Each struct in this module represents one system in our space_shooter game.
//!
//! The general rule for figuring out what goes in what system is to remember
//! <em>you can't eat your cake and have it too</em>.
//! For example, if you are modifying the transform (position) of a ship,
//! you can't even read the transform (position) of a laser or an asteroid.
//! So, generally, you want to keep movement and collision systems separated.
//!
//! Other than that you should try and do as much modification as possible
//! when you do have write access, because each pass of the system costs you time.
//! For example, you generally don't put the bounding of a ship/asteroid/laser
//! in a different system than the one that moves it.
//!
//! A system takes a set of common elements, such as components, entities and resources,
//! and performs some action with them.

mod asteroid;
mod laser;
mod laser_collision;
mod lives;
mod ship;
mod ship_collision;

pub use self::asteroid::AsteroidSystem;
pub use self::laser::LaserSystem;
pub use self::laser_collision::LaserCollisionSystem;
pub use self::lives::LivesSystem;
pub use self::ship::ShipSystem;
pub use self::ship_collision::ShipCollisionSystem;
