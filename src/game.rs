use bevy::prelude::*;

pub mod camera;
pub mod default_plugin_setup;

use camera::*;
use default_plugin_setup::*;

pub fn init() {
    App::new()
        .add_plugin(DefaultPluginSetup)
        .add_plugin(GameCameraPlugin)
        .run();
}
