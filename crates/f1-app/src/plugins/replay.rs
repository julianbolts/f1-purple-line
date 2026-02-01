//! Replay plugin for session playback.

use bevy::prelude::*;
use std::fs;
use std::path::Path;

use crate::resources::{AppState, AvailableSession, AvailableSessions, LoadSession, ReplayState, SessionData};

pub struct ReplayPlugin;

impl Plugin for ReplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SessionData>()
            .init_resource::<ReplayState>()
            .init_resource::<AvailableSessions>()
            .add_event::<LoadSession>()
            .add_systems(Update, (
                scan_sessions.run_if(in_state(AppState::Loading)),
                update_replay.run_if(in_state(AppState::Replay)),
                crate::systems::draw_circuit.run_if(in_state(AppState::Replay)),
                load_session,
            ));
    }
}

fn scan_sessions(
    mut available_sessions: ResMut<AvailableSessions>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Try multiple paths:
    // 1. "data" - if running from workspace root
    // 2. "../../data" - if running from crate root
    let paths = [Path::new("data"), Path::new("../../data")];
    let mut data_dir = Path::new("data");
    let mut found = false;

    for path in &paths {
        if path.exists() && path.is_dir() {
            data_dir = path;
            found = true;
            break;
        }
    }

    if !found {
        warn!("Could not find data directory. Checked: {:?}", paths);
        // Transition anyway so we don't get stuck, UI will just be empty
        next_state.set(AppState::SessionSelect);
        return;
    }
    
    info!("Scanning for sessions in {:?}", data_dir);

    if let Ok(entries) = fs::read_dir(data_dir) {
        let mut sessions = Vec::new();
        
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    // Format: 2025_austin_q -> "Austin 2025 Q"
                    let parts: Vec<&str> = file_name.split('_').collect();
                    let name = if parts.len() >= 3 {
                        let year = parts[0];
                        let circuit = parts[1]; // simplified, would need proper casing
                        let session = parts[2].to_uppercase();
                        // Simple capitalization for circuit
                        let circuit_cap = if let Some(c) = circuit.chars().next() {
                            format!("{}{}", c.to_uppercase(), &circuit[1..])
                        } else {
                            circuit.to_string()
                        };
                        format!("{} {} {}", circuit_cap, year, session)
                    } else {
                        file_name.to_string()
                    };

                    sessions.push(AvailableSession {
                        path: fs::canonicalize(&path).unwrap_or(path),
                        name,
                    });
                }
            }
        }
        
        // Sort explicitly to ensure consistent order (e.g. Austin Q before Austin R)
        sessions.sort_by(|a, b| a.name.cmp(&b.name));
        available_sessions.sessions = sessions;
        info!("Found {} sessions ", available_sessions.sessions.len());
    } else {
        warn!("Could not read data directory: {:?}", data_dir);
    }
    
    // Always transition to UI state
    next_state.set(AppState::SessionSelect);
}

fn load_session(
    mut commands: Commands,
    mut events: EventReader<LoadSession>,
    mut session_data: ResMut<SessionData>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in events.read() {
        info!("Loading session from {:?}", event.0);
        
        match f1_data::loader::load_session(&event.0) {
            Ok(session) => {
                info!("Loaded session: {} {} {}", session.year, session.circuit, session.session_type.as_str());
                session_data.session = Some(session);
                next_state.set(AppState::Replay);
            }
            Err(e) => {
                error!("Failed to load session: {:?}", e);
            }
        }
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
