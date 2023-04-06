use bevy::prelude::*;

#[derive(Reflect, Resource, Clone)]
#[reflect(Resource)]
pub struct GameOptions {
    pub pixel_scale: f32,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self { pixel_scale: 1.0 }
    }
}
