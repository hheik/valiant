use bevy::prelude::*;

pub const TILE_SIZE: f32 = 8.0;

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}

#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let ascii = assets.load("sprites/ascii.png");
    let atlas = TextureAtlas::from_grid(
        ascii,
        Vec2::splat(TILE_SIZE),
        16,
        16,
        Some(Vec2::splat(2.0)),
        Some(Vec2::splat(1.0)),
    );
    let atlas_handle = atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}
