use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct Player {
    pub name: String,
    pub velocity: f32,
}

impl Player {
    pub fn new(name: String, velocity: f32) -> Self {
        Player { name, velocity }
    }

    pub fn accelerate(&mut self, a: f32) {
        self.velocity += a;
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
