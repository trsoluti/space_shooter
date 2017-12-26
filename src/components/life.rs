use amethyst::ecs::{Component, DenseVecStorage};

pub struct Life {
    pub life_number: u8
}

impl Component for Life {
    type Storage = DenseVecStorage<Self>;
}
