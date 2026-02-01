//! Replay plugin for session playback.

use bevy::prelude::*;

use crate::resources::{AppState, ReplayState, SessionData};

pub struct ReplayPlugin;

impl Plugin for ReplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SessionData>()
            .init_resource::<ReplayState>()
            .add_systems(Update, update_replay.run_if(in_state(AppState::Replay)));
    }
}

fn update_replay(
    time: Res<Time>,
    mut replay_state: ResMut<ReplayState>,
    session_data: Res<SessionData>,
) {
    if !replay_state.playing {
        return;
    }

    let Some(_session) = &session_data.session else {
        return;
    };

    // Advance replay time
    replay_state.current_time += time.delta_secs_f64() * replay_state.speed as f64;

    // TODO: Update car positions based on current_time
    // This will interpolate between telemetry samples
}
