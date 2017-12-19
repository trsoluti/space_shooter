
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::{LocalTransform};
use rand::Rng;
use rand::ThreadRng;
use config::GAME_CONFIGURATION;

#[derive(Clone)]
pub struct Asteroid {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
    pub is_destroyed: bool,
}

impl Component for Asteroid {
    type Storage = DenseVecStorage<Self>;
}

impl Asteroid {
    pub fn locate(&self, screen_width: f32, screen_height: f32, rng: &mut ThreadRng)->LocalTransform {
        let max_width = screen_width - self.width;
        let min_height = screen_height + GAME_CONFIGURATION.wait_for_first_asteroid * GAME_CONFIGURATION.asteroid_velocity;
        let max_height = min_height + screen_height / GAME_CONFIGURATION.asteroid_velocity;
        let pos_x = rng.next_f32()*max_width;
        let pos_y = rng.next_f32()*(max_height-min_height);

        let mut local_transform = LocalTransform::default();
        local_transform.translation = Vector3::new(pos_x, pos_y, 0.0);
        local_transform.scale = Vector3::new(0.1, 0.1, 1.0);
        local_transform
    }
}
