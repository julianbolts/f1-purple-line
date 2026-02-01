//! Shared resources for the application.

use bevy::prelude::*;
use f1_data::{CircuitTurns, Session};

/// Application state machine.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    SessionSelect,
    Replay,
    Analysis,
}

/// Currently loaded session data.
#[derive(Resource, Default)]
pub struct SessionData {
    pub session: Option<Session>,
}

/// Replay control state.
#[derive(Resource)]
pub struct ReplayState {
    /// Current playback time in seconds
    pub current_time: f64,
    /// Playback speed multiplier (1.0 = real-time)
    pub speed: f32,
    /// Whether replay is playing or paused
    pub playing: bool,
    /// Selected drivers to display
    pub selected_drivers: Vec<String>,
}

impl Default for ReplayState {
    fn default() -> Self {
        Self {
            current_time: 0.0,
            speed: 1.0,
            playing: false,
            selected_drivers: Vec::new(),
        }
    }
}

/// Turn definitions for the current circuit.
#[derive(Resource, Default)]
pub struct TurnData {
    pub turns: Option<CircuitTurns>,
}

/// Currently selected turn for analysis.
#[derive(Resource, Default)]
pub struct SelectedTurn {
    pub turn_name: Option<String>,
}

/// Drivers currently being compared.
#[derive(Resource, Default)]
pub struct ComparisonDrivers {
    pub driver_codes: Vec<String>,
}
