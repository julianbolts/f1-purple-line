//! Core data types for F1 telemetry.

use chrono::{DateTime, Utc};
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// A single telemetry sample from a car.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySample {
    /// Position in world coordinates (meters)
    pub position: Vec3,
    /// Session time when this sample was recorded
    pub time: f64,
    /// Throttle application (0-100%)
    pub throttle: u8,
    /// Brake application (0 or 100 in F1 data)
    pub brake: u8,
    /// Current gear (0 = neutral, 1-8 = forward gears)
    pub gear: u8,
    /// Speed in km/h
    pub speed: u16,
    /// DRS status
    pub drs: DrsStatus,
}

/// DRS (Drag Reduction System) status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DrsStatus {
    #[default]
    Off,
    Eligible,
    Active,
}

/// A complete lap with all telemetry samples.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lap {
    /// Lap number (1-indexed)
    pub number: u32,
    /// Lap time in seconds (None if incomplete)
    pub lap_time: Option<f64>,
    /// Sector times [S1, S2, S3] in seconds
    pub sector_times: [Option<f64>; 3],
    /// All telemetry samples for this lap
    pub samples: Vec<TelemetrySample>,
    /// Whether this lap is valid (no track limits, etc.)
    pub is_valid: bool,
}

/// Driver information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    /// Three-letter driver abbreviation (e.g., "VER", "HAM")
    pub code: String,
    /// Full driver name
    pub name: String,
    /// Car number
    pub number: u32,
    /// Team name
    pub team: String,
    /// Team color as hex (e.g., "#FF0000")
    pub team_color: String,
}

/// A driver's session data containing all their laps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverSession {
    pub driver: Driver,
    pub laps: Vec<Lap>,
}

/// Session type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionType {
    Practice1,
    Practice2,
    Practice3,
    Qualifying,
    Sprint,
    Race,
}

impl SessionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Practice1 => "FP1",
            Self::Practice2 => "FP2",
            Self::Practice3 => "FP3",
            Self::Qualifying => "Q",
            Self::Sprint => "Sprint",
            Self::Race => "Race",
        }
    }
}

/// Complete session data for a race weekend event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Year of the season
    pub year: u32,
    /// Circuit name
    pub circuit: String,
    /// Circuit short name / location
    pub circuit_short: String,
    /// Type of session
    pub session_type: SessionType,
    /// Session date/time
    pub date: DateTime<Utc>,
    /// All driver sessions
    pub drivers: Vec<DriverSession>,
}

impl Session {
    /// Get a driver's session data by their code.
    pub fn get_driver(&self, code: &str) -> Option<&DriverSession> {
        self.drivers.iter().find(|d| d.driver.code == code)
    }

    /// Get all driver codes in this session.
    pub fn driver_codes(&self) -> Vec<&str> {
        self.drivers.iter().map(|d| d.driver.code.as_str()).collect()
    }
}
