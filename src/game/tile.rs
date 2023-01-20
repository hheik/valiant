use bevy::prelude::*;

use crate::util::Vector2I;

use super::ascii::TILE_SIZE;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Position>()
            .register_type::<Collision>()
            .add_system(position_system);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Position(pub Vector2I);

fn position_system(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        let pos = Vec3::from(position.0) * TILE_SIZE + Vec3::new(0.0, 0.0, transform.translation.z);
        transform.translation = pos;
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Collision;
