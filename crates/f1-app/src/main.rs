//! F1 Purple Line - Telemetry Visualization and Analysis Tool

use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod systems;

use plugins::{CameraPlugin, ReplayPlugin, UiPlugin};
use resources::AppState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "F1 Purple Line".into(),
                resolution: (1600., 900.).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins((CameraPlugin, ReplayPlugin, UiPlugin))
        .run();
}
