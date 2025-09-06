use bevy::prelude::*;
use psp_background::WavePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WavePlugin::default())
        .run();
}
