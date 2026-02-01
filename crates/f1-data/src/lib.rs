//! F1 telemetry data types and analysis library.
//!
//! This crate provides core data structures for F1 telemetry data
//! and analysis tools for turn segmentation and lap comparison.

pub mod analysis;
pub mod loader;
pub mod turn;
pub mod types;

pub use turn::{BoundingBox, CircuitTurns, Turn, TurnSegment};
pub use types::*;
