use bevy::prelude::*;

pub const TILE_SIZE: f32 = 8.0;

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_ascii.in_base_set(StartupSet::PreStartup))
            .add_system(ascii_system);
    }
}

#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

#[derive(Default)]
pub struct AsciiBundle {
    pub ascii: AsciiIndex,
    pub sprite_sheet: SpriteSheetBundle,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AsciiIndex(u8);

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

fn ascii_system(mut query: Query<(&mut TextureAtlasSprite, &AsciiIndex), Changed<AsciiIndex>>) {
    for (mut sprite_sheet, ascii_index) in query.iter_mut() {
        sprite_sheet.index = ascii_index.0 as usize;
    }
}
