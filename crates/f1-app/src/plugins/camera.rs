//! Camera plugin for 2D/3D visualization.

use bevy::prelude::*;

use crate::components::MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // Spawn a 2D camera for track visualization
    // F1 tracks are typically viewed from above
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));
}
