use bevy::{prelude::*, window::WindowResolution};

use super::config::GameOptions;

pub struct DefaultPluginSetup;

impl Plugin for DefaultPluginSetup {
    fn build(&self, app: &mut App) {
        let options = app
            .world
            .get_resource::<GameOptions>()
            .cloned()
            .unwrap_or_default();

        app.insert_resource(ClearColor(Color::BLACK)).add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            800.0 * options.pixel_scale,
                            600.0 * options.pixel_scale,
                        ),
                        title: "Valiant".to_string(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
    }
}
