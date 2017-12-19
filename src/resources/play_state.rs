
pub enum PlayStateEnum {
    PlayOngoing,
    PlayComplete,
}

pub struct PlayState {
    pub current_state: PlayStateEnum,
}
