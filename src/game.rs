use bevy::prelude::*;

pub mod ascii;
pub mod camera;
pub mod collision;
pub mod debug;
pub mod default_plugin_setup;
pub mod position;

use ascii::*;
use camera::*;
use collision::*;
use default_plugin_setup::*;
use position::*;

use crate::util::Vector2I;

use self::debug::DebugPlugin;

pub fn init() {
    App::new()
        .add_plugin(DefaultPluginSetup)
        .add_plugin(AsciiPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(PositionPlugin)
        .add_startup_system(setup_player)
        .add_startup_system(setup_map)
        .run();
}

fn setup_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::BLUE;

    commands
        .spawn(Position(Vector2I::new(1, 0)))
        .insert(SpriteSheetBundle {
            texture_atlas: ascii.0.clone(),
            sprite,
            ..default()
        })
        .insert(CameraFollow {
            movement: FollowMovement::Instant,
            priority: 0,
        })
        .insert(Name::new("Player"));
}

fn setup_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    for y in -4..4 {
        for x in -7..7 {
            let is_wall = y == -4 || y == 3 || x == -7 || x == 6;
            let mut sprite = TextureAtlasSprite::new(if is_wall { 219 } else { 7 });
            sprite.color = Color::GRAY;
            let ent = commands
                .spawn(Position(Vector2I::new(x, y)))
                .insert(SpriteSheetBundle {
                    texture_atlas: ascii.0.clone(),
                    sprite,
                    ..default()
                })
                .insert(Name::new(if is_wall { "Wall" } else { "Floor" }))
                .id();
            if is_wall {
                commands.get_entity(ent).unwrap().insert(Collision);
            }
        }
    }
}
