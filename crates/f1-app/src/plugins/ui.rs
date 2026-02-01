//! UI plugin for controls and information display.

use bevy::prelude::*;

use crate::resources::{ComparisonDrivers, SelectedTurn, TurnData};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnData>()
            .init_resource::<SelectedTurn>()
            .init_resource::<ComparisonDrivers>()
            .add_systems(Startup, setup_ui);
    }
}

fn setup_ui(mut _commands: Commands) {
    // TODO: Set up UI elements
    // - Driver selection panel
    // - Playback controls
    // - Telemetry display
    // - Turn analysis panel
}
