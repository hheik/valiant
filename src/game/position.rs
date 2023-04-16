use bevy::prelude::*;

use crate::util::Vector2I;

pub const TILE_SIZE: f32 = 16.0;

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Position>().add_system(position_system);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Position(pub Vector2I);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(Vector2I::new(x, y))
    }
}

impl From<IVec2> for Position {
    fn from(value: IVec2) -> Self {
        Self(Vector2I::from(value))
    }
}

fn position_system(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.0.x as f32 * TILE_SIZE;
        transform.translation.y = position.0.y as f32 * TILE_SIZE;
    }
}
