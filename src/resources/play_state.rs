
/// The play state of our game
///
/// This resource stores the number of lives the player has currently.
///
/// Every time the ship collides with an asteroid, the number of lives
/// is reduced.
///
/// The lives system uses this resource to determine how many
/// life icons to display on the UI level.
///
/// The game state uses this resource to determine when to end the game.
pub struct PlayState {
    /// Number of lives the player has currently
    pub lives: u8,
}
