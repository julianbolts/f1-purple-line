//! ECS Components for the F1 visualization.

use bevy::prelude::*;

/// Marker component for a car entity being visualized.
#[derive(Component)]
pub struct Car {
    /// Driver code (e.g., "VER")
    pub driver_code: String,
    /// Current lap number
    pub current_lap: u32,
    /// Current sample index within the lap
    pub sample_index: usize,
}

/// Component storing the car's trail (position history for visualization).
#[derive(Component, Default)]
pub struct Trail {
    /// Recent positions for drawing the trail
    pub positions: Vec<Vec3>,
    /// Maximum trail length
    pub max_length: usize,
}

impl Trail {
    pub fn new(max_length: usize) -> Self {
        Self {
            positions: Vec::with_capacity(max_length),
            max_length,
        }
    }

    pub fn push(&mut self, pos: Vec3) {
        if self.positions.len() >= self.max_length {
            self.positions.remove(0);
        }
        self.positions.push(pos);
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }
}

/// Component for telemetry display (speed, gear, throttle, etc.)
#[derive(Component)]
pub struct TelemetryDisplay {
    pub driver_code: String,
}

/// Marker for the main camera.
#[derive(Component)]
pub struct MainCamera;

/// Component for entities that should follow a specific car.
#[derive(Component)]
pub struct FollowCar {
    pub driver_code: String,
    pub offset: Vec3,
}

/// Marker for track visualization entities.
#[derive(Component)]
pub struct TrackEntity;

/// Marker for turn boundary visualization.
#[derive(Component)]
pub struct TurnBoundary {
    pub turn_name: String,
}
