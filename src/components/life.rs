use amethyst::ecs::prelude::{Component, DenseVecStorage};

/// A component to represent a life the player has
///
/// It's connected to a life sprite shown in the UI layer.
/// When the play state number of lives fall below
/// the life number of this component,
/// the system will remove the associated life entity.
pub struct Life {
    /// The life number this component represents
    pub life_number: u8,
}

impl Component for Life {
    type Storage = DenseVecStorage<Self>;
}
