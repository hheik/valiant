use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

pub mod actor;
pub mod camera;
pub mod config;
pub mod debug;
pub mod default_plugin_setup;
pub mod ldtk;
pub mod position;
pub mod tile;

use self::{
    actor::Actor,
    camera::*,
    position::{Position, TILE_SIZE},
};

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
        .add_startup_system(setup_player)
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

fn setup_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let atlas = TextureAtlas::from_grid(
        assets.load("sprites/adventurer_1.png"),
        Vec2::splat(TILE_SIZE),
        1,
        1,
        Some(Vec2::splat(0.0)),
        Some(Vec2::splat(0.0)),
    );
    let atlas_handle = atlases.add(atlas);

    commands
        .spawn(Actor)
        .insert(Position::new(11, 4))
        .insert(SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite {
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..TextureAtlasSprite::new(0)
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        })
        .insert(CameraFollow {
            movement: FollowMovement::Instant,
            priority: 1,
        })
        .insert(Name::new("Player 1"));
}
