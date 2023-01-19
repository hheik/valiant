use bevy::prelude::*;

pub mod ascii;
pub mod camera;
pub mod debug;
pub mod default_plugin_setup;

use ascii::*;
use camera::*;
use default_plugin_setup::*;

use self::debug::DebugPlugin;

pub fn init() {
    App::new()
        .add_plugin(DefaultPluginSetup)
        .add_plugin(AsciiPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(setup_player)
        .run();
}

pub fn setup_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::BLUE;

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: ascii.0.clone(),
            sprite,
            ..default()
        })
        .insert(Name::new("Player"));
}
