//! Lap and turn analysis functionality.

use crate::{Lap, TelemetrySample, Turn, TurnSegment};

/// Time spent in a turn or segment.
#[derive(Debug, Clone)]
pub struct SegmentTime {
    /// Time in seconds
    pub time: f64,
    /// Entry speed (km/h)
    pub entry_speed: u16,
    /// Minimum speed in segment (km/h)
    pub min_speed: u16,
    /// Exit speed (km/h)
    pub exit_speed: u16,
}

/// Complete turn analysis for a single lap.
#[derive(Debug, Clone)]
pub struct TurnAnalysis {
    pub lap_number: u32,
    /// Total time through the turn
    pub total_time: Option<SegmentTime>,
    /// Approach segment analysis
    pub approach: Option<SegmentTime>,
    /// Apex segment analysis
    pub apex: Option<SegmentTime>,
    /// Exit segment analysis
    pub exit: Option<SegmentTime>,
}

impl TurnAnalysis {
    /// Analyze a lap through a specific turn.
    pub fn analyze(lap: &Lap, turn: &Turn) -> Self {
        let samples = turn.filter_samples(&lap.samples);

        Self {
            lap_number: lap.number,
            total_time: compute_segment_time(&samples),
            approach: compute_segment_time(&turn.filter_segment(&lap.samples, TurnSegment::Approach)),
            apex: compute_segment_time(&turn.filter_segment(&lap.samples, TurnSegment::Apex)),
            exit: compute_segment_time(&turn.filter_segment(&lap.samples, TurnSegment::Exit)),
        }
    }
}

/// Compute timing and speed data for a set of samples.
fn compute_segment_time(samples: &[&TelemetrySample]) -> Option<SegmentTime> {
    if samples.len() < 2 {
        return None;
    }

    let first = samples.first()?;
    let last = samples.last()?;
    let time = last.time - first.time;

    let min_speed = samples.iter().map(|s| s.speed).min().unwrap_or(0);

    Some(SegmentTime {
        time,
        entry_speed: first.speed,
        min_speed,
        exit_speed: last.speed,
    })
}

/// Result of comparing laps through a turn.
#[derive(Debug)]
pub struct TurnComparison {
    pub turn_name: String,
    /// Analyses sorted by total turn time (fastest first)
    pub analyses: Vec<TurnAnalysis>,
    /// Whether the fastest overall lap is also fastest in all segments
    pub uniform_fastest: bool,
    /// Fastest lap for each segment (if different from overall fastest)
    pub segment_bests: SegmentBests,
}

/// Best lap numbers for each segment.
#[derive(Debug, Default)]
pub struct SegmentBests {
    pub approach: Option<u32>,
    pub apex: Option<u32>,
    pub exit: Option<u32>,
}

impl TurnComparison {
    /// Compare multiple laps through a turn.
    pub fn compare(laps: &[Lap], turn: &Turn) -> Self {
        let mut analyses: Vec<TurnAnalysis> = laps
            .iter()
            .map(|lap| TurnAnalysis::analyze(lap, turn))
            .filter(|a| a.total_time.is_some())
            .collect();

        // Sort by total turn time (fastest first)
        analyses.sort_by(|a, b| {
            let time_a = a.total_time.as_ref().map(|t| t.time).unwrap_or(f64::MAX);
            let time_b = b.total_time.as_ref().map(|t| t.time).unwrap_or(f64::MAX);
            time_a.partial_cmp(&time_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        let segment_bests = compute_segment_bests(&analyses);
        let fastest_lap = analyses.first().map(|a| a.lap_number);

        // Check if overall fastest is also fastest in all segments
        let uniform_fastest = fastest_lap.map_or(false, |fl| {
            segment_bests.approach.map_or(true, |l| l == fl)
                && segment_bests.apex.map_or(true, |l| l == fl)
                && segment_bests.exit.map_or(true, |l| l == fl)
        });

        Self {
            turn_name: turn.name.clone(),
            analyses,
            uniform_fastest,
            segment_bests,
        }
    }
}

fn compute_segment_bests(analyses: &[TurnAnalysis]) -> SegmentBests {
    let find_best = |get_time: fn(&TurnAnalysis) -> Option<f64>| -> Option<u32> {
        analyses
            .iter()
            .filter_map(|a| get_time(a).map(|t| (a.lap_number, t)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(lap, _)| lap)
    };

    SegmentBests {
        approach: find_best(|a| a.approach.as_ref().map(|s| s.time)),
        apex: find_best(|a| a.apex.as_ref().map(|s| s.time)),
        exit: find_best(|a| a.exit.as_ref().map(|s| s.time)),
    }
}
