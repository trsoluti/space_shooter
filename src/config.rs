
/// "Constants" that control the game mechanics
pub struct Configuration {
    pub ship_velocity: f32,
    pub asteroid_velocity: f32,
    pub wait_for_first_asteroid: f32,
}

pub const SHIP_VELOCITY: f32 = 75.0;
pub const ASTEROID_VELOCITY: f32 = 0.02;
pub const WAIT_FOR_FIRST_ASTEROID: f32 = 2.0;

lazy_static! {
    pub static ref GAME_CONFIGURATION: Configuration = Configuration {
     ship_velocity: SHIP_VELOCITY,
     asteroid_velocity: ASTEROID_VELOCITY,
     wait_for_first_asteroid: WAIT_FOR_FIRST_ASTEROID,
     };
}

