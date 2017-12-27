//! Components for the game
//!
//! Each struct in this module represents a component of our space_shooter game.
//!
//! Note all components should be cloneable, so you can have more copies of
//! a component when you need them. That's just good practice.

mod ship;
mod asteroid;
mod laser;
mod life;

pub use self::ship::Ship;
pub use self::asteroid::Asteroid;
pub use self::laser::Laser;
pub use self::life::Life;