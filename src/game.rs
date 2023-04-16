use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

pub mod actor;
pub mod adventurer;
pub mod camera;
pub mod config;
pub mod debug;
pub mod default_plugin_setup;
pub mod ldtk;
pub mod position;
pub mod tile;

pub fn init() {
    App::new()
        .register_type::<config::GameOptions>()
        .insert_resource(config::GameOptions::default())
        .add_plugin(default_plugin_setup::DefaultPluginSetup)
        .add_plugin(ldtk::LdtkHelperPlugin)
        .insert_resource(LevelSelection::Index(0))
        .add_plugin(camera::GameCameraPlugin)
        .add_plugin(position::PositionPlugin)
        .add_plugin(tile::TilePlugin)
        .add_plugin(actor::ActorPlugin)
        .add_plugin(adventurer::AdventurerPlugin)
        .add_startup_system(setup_level)
        // .add_plugin(debug::DebugPlugin)
        .run();
}

fn setup_level(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.load("levels/world.ldtk"),
        ..default()
    });
}
