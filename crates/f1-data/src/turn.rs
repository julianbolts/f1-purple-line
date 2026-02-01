//! Turn definition and segmentation.
//!
//! A turn is defined by a bounding area on the track. Each turn is divided
//! into three segments for analysis: approach, apex, and exit.

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::TelemetrySample;

/// A 2D bounding box for defining track regions.
/// Uses X and Z coordinates (Y is typically elevation in F1 data).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    /// Minimum X coordinate
    pub min_x: f32,
    /// Maximum X coordinate
    pub max_x: f32,
    /// Minimum Z coordinate
    pub min_z: f32,
    /// Maximum Z coordinate
    pub max_z: f32,
}

impl BoundingBox {
    pub fn new(min_x: f32, max_x: f32, min_z: f32, max_z: f32) -> Self {
        Self {
            min_x,
            max_x,
            min_z,
            max_z,
        }
    }

    /// Check if a position is within this bounding box (ignoring Y).
    pub fn contains(&self, pos: Vec3) -> bool {
        pos.x >= self.min_x && pos.x <= self.max_x && pos.z >= self.min_z && pos.z <= self.max_z
    }

    /// Create from two corner points.
    pub fn from_corners(corner1: Vec3, corner2: Vec3) -> Self {
        Self {
            min_x: corner1.x.min(corner2.x),
            max_x: corner1.x.max(corner2.x),
            min_z: corner1.z.min(corner2.z),
            max_z: corner1.z.max(corner2.z),
        }
    }
}

/// Segment of a turn for detailed analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnSegment {
    Approach,
    Apex,
    Exit,
}

/// A turn definition with its bounding boxes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turn {
    /// Turn identifier (e.g., "T1", "Maggots")
    pub name: String,
    /// Overall turn bounding box
    pub bounds: BoundingBox,
    /// Approach segment bounds (entry into the turn)
    pub approach: BoundingBox,
    /// Apex segment bounds (middle of the turn)
    pub apex: BoundingBox,
    /// Exit segment bounds (exiting the turn)
    pub exit: BoundingBox,
}

impl Turn {
    /// Filter samples that are within this turn's overall bounds.
    pub fn filter_samples<'a>(
        &self,
        samples: &'a [TelemetrySample],
    ) -> Vec<&'a TelemetrySample> {
        samples
            .iter()
            .filter(|s| self.bounds.contains(s.position))
            .collect()
    }

    /// Get samples for a specific segment of the turn.
    pub fn filter_segment<'a>(
        &self,
        samples: &'a [TelemetrySample],
        segment: TurnSegment,
    ) -> Vec<&'a TelemetrySample> {
        let bounds = match segment {
            TurnSegment::Approach => &self.approach,
            TurnSegment::Apex => &self.apex,
            TurnSegment::Exit => &self.exit,
        };
        samples
            .iter()
            .filter(|s| bounds.contains(s.position))
            .collect()
    }

    /// Determine which segment a position is in (if any).
    pub fn classify_position(&self, pos: Vec3) -> Option<TurnSegment> {
        if self.approach.contains(pos) {
            Some(TurnSegment::Approach)
        } else if self.apex.contains(pos) {
            Some(TurnSegment::Apex)
        } else if self.exit.contains(pos) {
            Some(TurnSegment::Exit)
        } else {
            None
        }
    }
}

/// Collection of turns for a circuit.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitTurns {
    pub circuit: String,
    pub turns: Vec<Turn>,
}

impl CircuitTurns {
    pub fn get_turn(&self, name: &str) -> Option<&Turn> {
        self.turns.iter().find(|t| t.name == name)
    }
}
