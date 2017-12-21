
/// "Constants" that control the game mechanics
pub struct Configuration {
    pub ship_velocity: f32,
    pub asteroid_velocity: f32,
    pub wait_for_first_asteroid: f32,
    pub asteroid_density: f32,
    pub laser_velocity: f32,
}

pub const SHIP_VELOCITY: f32 = 75.0;
pub const ASTEROID_VELOCITY: f32 = 5.0;
pub const LASER_VELOCITY: f32 = 100.0;
pub const WAIT_FOR_FIRST_ASTEROID: f32 = 2.0;
pub const ASTEROID_DENSITY: f32 = 0.3;

lazy_static! {
    pub static ref GAME_CONFIGURATION: Configuration = Configuration {
     ship_velocity: SHIP_VELOCITY,
     asteroid_velocity: ASTEROID_VELOCITY,
     asteroid_density: ASTEROID_DENSITY,
     wait_for_first_asteroid: WAIT_FOR_FIRST_ASTEROID,
     laser_velocity: LASER_VELOCITY,
     };
}

