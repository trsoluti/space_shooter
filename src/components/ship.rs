
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Ship;

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}
