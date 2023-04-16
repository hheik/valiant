use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::LdtkLevel;

use crate::util::Vector2I;

use super::{
    actor::Actor,
    camera::CameraFollow,
    ldtk::{EntityInstanceAdded, FieldValueGetter},
    position::{Position, TILE_SIZE},
};

pub struct AdventurerPlugin;

impl Plugin for AdventurerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Adventurer>()
            .add_system(adventurer_spawner);
    }
}

#[derive(Reflect, Default)]
pub struct BaseStats {
    spirit: i32,
    intellect: i32,
    power: i32,
    stealth: i32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Adventurer {
    base_stats: BaseStats,
    name: String,
}

fn adventurer_spawner(
    mut commands: Commands,
    mut events: EventReader<EntityInstanceAdded>,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut transform_query: Query<&mut Transform>,
    parent_query: Query<&Parent>,
    level_query: Query<&Handle<LdtkLevel>>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for event in events
        .iter()
        .filter(|e| e.instance.identifier == "ADVENTURER")
    {
        let instance = &event.instance;

        let parent = match parent_query.get(event.entity) {
            Ok(parent) => parent,
            Err(err) => {
                error!("parent_query: {err}");
                continue;
            }
        };

        let level = match level_query.get(parent.get()) {
            Ok(level) => level,
            Err(err) => {
                error!("level_query: {err}");
                continue;
            }
        };

        let level = match ldtk_levels.get(level) {
            Some(level) => &level.level,
            None => {
                error!("Could not find level from handle!");
                continue;
            }
        };

        let depth = transform_query
            .get_mut(event.entity)
            .ok()
            .map_or(0.0, |t| t.translation.z);
        let position = Position(Vector2I {
            x: instance.grid.x,
            y: (level.px_hei as f32 / TILE_SIZE - instance.grid.y as f32) as i32,
        });

        commands.entity(parent.get()).with_children(|builder| {
            let atlas = TextureAtlas::from_grid(
                assets.load("sprites/adventurers.png"),
                Vec2::splat(16.),
                1,
                1,
                None,
                None,
            );
            let atlas_handle = atlases.add(atlas);
            let name = instance
                .find_string("NAME")
                .unwrap_or("<adventurer>".to_string());

            builder.spawn((
                SpriteSheetBundle {
                    texture_atlas: atlas_handle,
                    sprite: TextureAtlasSprite {
                        index: 0,
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, depth),
                    ..default()
                },
                position,
                Adventurer {
                    name: name.clone(),
                    base_stats: BaseStats {
                        spirit: instance.find_i32("SPIRIT").unwrap_or(0),
                        intellect: instance.find_i32("INTELLECT").unwrap_or(0),
                        power: instance.find_i32("POWER").unwrap_or(0),
                        stealth: instance.find_i32("STEALTH").unwrap_or(0),
                    },
                },
                Actor::default(),
                CameraFollow::default(),
                Name::new(name.clone()),
            ));
        });
    }
}
