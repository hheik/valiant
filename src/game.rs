use bevy::prelude::*;

pub mod ascii;
pub mod camera;
pub mod debug;
pub mod default_plugin_setup;
pub mod position;

use ascii::*;
use camera::*;
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
        .run();
}

pub fn setup_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
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
