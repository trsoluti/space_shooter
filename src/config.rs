//! The set of items used by the Game Designer to tune the game after the coding is complete (e.g. player speed)

use amethyst::config::Config;

/// "Constants" that control the game mechanics
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameConfiguration {
    /// The effect each key press has on the ship's speed
    #[serde(default)]
    pub ship_thrust: f32,
    /// asteroid vertical velocity
    #[serde(default)]
    pub asteroid_velocity: f32,
    /// how long to wait before the first asteroid falls (sec)
    #[serde(default)]
    pub wait_for_first_asteroid: f32,
    /// how close the asteroids are together
    #[serde(default)]
    pub asteroid_density: f32,
    /// laser vertical velocity
    #[serde(default)]
    pub laser_velocity: f32,
    /// how long to wait after firing a laser before can fire again
    #[serde(default)]
    pub trigger_reset_timeout: f32,
}

// Default values
pub const SHIP_THRUST: f32 = 0.02;
pub const ASTEROID_VELOCITY: f32 = 5.0;
pub const LASER_VELOCITY: f32 = 60.0;
pub const WAIT_FOR_FIRST_ASTEROID: f32 = 2.0;
pub const ASTEROID_DENSITY: f32 = 0.3;
pub const TRIGGER_RESET_TIMEOUT: f32 = 0.5;

impl Default for GameConfiguration {
    fn default() -> Self {
        GameConfiguration {
            ship_thrust: SHIP_THRUST,
            asteroid_velocity: ASTEROID_VELOCITY,
            asteroid_density: ASTEROID_DENSITY,
            wait_for_first_asteroid: WAIT_FOR_FIRST_ASTEROID,
            laser_velocity: LASER_VELOCITY,
            trigger_reset_timeout: TRIGGER_RESET_TIMEOUT,
        }
    }
}

lazy_static! {
    /// The actual values for the [game configuration](struct.GameConfiguration.html)</a>.
    ///
    /// The game configuration is automatically loaded on startup
    /// from the file "game_config.ron" in resources.
    ///
    /// It's a good pattern for managing the game configuration
    /// so that the game designer can change parameters and balance
    /// the game without having to recompile the code.
    ///
    /// This variable looks to the remaining code as if it were a set of constants.
    pub static ref GAME_CONFIGURATION: GameConfiguration = {
        let game_config_path = format!(
            "{}/resources/game_config.ron",
            env!("CARGO_MANIFEST_DIR")
        );
        GameConfiguration::load(&game_config_path)
    };

}
