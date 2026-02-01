//! Input handling systems.

use bevy::prelude::*;

use crate::resources::ReplayState;

/// Toggle play/pause with spacebar.
pub fn handle_playback_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut replay_state: ResMut<ReplayState>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        replay_state.playing = !replay_state.playing;
    }

    // Speed controls
    if keyboard.just_pressed(KeyCode::BracketRight) {
        replay_state.speed = (replay_state.speed * 2.0).min(16.0);
    }
    if keyboard.just_pressed(KeyCode::BracketLeft) {
        replay_state.speed = (replay_state.speed / 2.0).max(0.25);
    }
}
