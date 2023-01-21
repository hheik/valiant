use bevy::prelude::*;

pub mod actor;
pub mod ascii;
pub mod camera;
pub mod debug;
pub mod default_plugin_setup;
pub mod position;
pub mod tile;

use actor::*;
use ascii::*;
use camera::*;
use default_plugin_setup::*;
use position::*;
use tile::*;

use crate::util::Vector2I;

use self::debug::DebugPlugin;

pub fn init() {
    App::new()
        .add_plugin(DefaultPluginSetup)
        .add_plugin(AsciiPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(PositionPlugin)
        .add_plugin(TilePlugin)
        .add_plugin(ActorPlugin)
        .add_startup_system(setup_player)
        .add_startup_system(setup_map)
        .add_plugin(DebugPlugin)
        .run();
}

fn setup_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    commands
        .spawn(Actor)
        .insert(Position(Vector2I::new(1, 0)))
        .insert(SpriteSheetBundle {
            texture_atlas: ascii.0.clone(),
            sprite: TextureAtlasSprite {
                color: Color::BLUE,
                ..TextureAtlasSprite::new(1)
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        })
        .insert(CameraFollow {
            movement: FollowMovement::Instant,
            priority: 1,
        })
        .insert(Name::new("Player 1"));

    commands
        .spawn(Actor)
        .insert(Position(Vector2I::new(3, -2)))
        .insert(SpriteSheetBundle {
            texture_atlas: ascii.0.clone(),
            sprite: TextureAtlasSprite {
                color: Color::RED,
                ..TextureAtlasSprite::new(2)
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        })
        .insert(CameraFollow {
            movement: FollowMovement::Instant,
            priority: 0,
        })
        .insert(Name::new("Player 2"));
}

fn setup_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    for y in -4..4 {
        for x in -7..7 {
            let is_wall = y == -4 || y == 3 || x == -7 || x == 6;
            let mut sprite = TextureAtlasSprite::new(if is_wall { 219 } else { 22 });
            sprite.color = Color::GRAY;
            let ent = commands
                .spawn(Position(Vector2I::new(x, y)))
                .insert(SpriteSheetBundle {
                    texture_atlas: ascii.0.clone(),
                    sprite,
                    ..default()
                })
                .insert(Name::new(format!(
                    "{} tile",
                    if is_wall { "Wall" } else { "Floor" },
                )))
                .id();
            if is_wall {
                commands.get_entity(ent).unwrap().insert(Collision);
            }
        }
    }
}
