
/// "Constants" that control the game mechanics
pub struct Configuration {
    pub ship_velocity: f32,
}

pub const SHIP_VELOCITY: f32 = 75.0;

lazy_static! {
    pub static ref GAME_CONFIGURATION: Configuration = Configuration { ship_velocity: SHIP_VELOCITY };
}

