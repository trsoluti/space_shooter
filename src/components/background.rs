
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Background;

impl Component for Background {
    type Storage = DenseVecStorage<Self>;
}
