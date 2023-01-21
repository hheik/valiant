use bevy::prelude::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Collision>();
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Collision;
