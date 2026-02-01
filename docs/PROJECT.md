# F1 Purple Line - Project Architecture

## Overview

F1 Purple Line is a telemetry visualization and analysis tool for Formula 1 racing data. It enables:

1. **Replay**: Visualize car positions through a session with playback controls
2. **Comparison**: Compare multiple drivers' laps side-by-side
3. **Turn Analysis**: Define track segments and analyze performance through turns

The name "Purple Line" refers to F1's purple sector times—the fastest times set by any driver.

## Inspiration

This project is a Rust/Bevy reimplementation of [f1-race-replay](https://github.com/IAmTomShaw/f1-race-replay), which uses Python and the [fastf1](https://docs.fastf1.dev/) library.

---

## Workspace Structure

```
f1-purple-line/
├── Cargo.toml              # Workspace manifest with shared dependencies
├── CLAUDE.md               # AI agent quick-start context
├── crates/
│   ├── f1-data/            # Core library (pure Rust, no Bevy)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs      # Public exports
│   │       ├── types.rs    # Data structures
│   │       ├── loader.rs   # JSON deserialization
│   │       ├── turn.rs     # Turn/segment definitions
│   │       └── analysis.rs # Comparison algorithms
│   └── f1-app/             # Bevy application
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── components.rs
│           ├── resources.rs
│           ├── plugins/
│           │   ├── mod.rs
│           │   ├── camera.rs
│           │   ├── replay.rs
│           │   └── ui.rs
│           └── systems/
│               ├── mod.rs
│               ├── input.rs
│               └── rendering.rs
├── tools/
│   ├── fetch_session.py    # Data export script
│   └── requirements.txt    # Python dependencies
├── data/                   # Session JSON files
└── docs/
    ├── PROJECT.md          # This file
    └── DEV_LOG.md          # Development history
```

---

## Crate: f1-data

Pure Rust library with no Bevy dependency. Can be used in CLI tools or other contexts.

### Core Types (`types.rs`)

```rust
TelemetrySample {
    position: Vec3,      // X/Z = track plane, Y = elevation
    time: f64,           // Session time in seconds
    throttle: u8,        // 0-100%
    brake: u8,           // 0 or 100 (binary in F1 data)
    gear: u8,            // 0=neutral, 1-8=forward
    speed: u16,          // km/h
    drs: DrsStatus,      // Off, Eligible, Active
}

Lap {
    number: u32,
    lap_time: Option<f64>,
    sector_times: [Option<f64>; 3],
    samples: Vec<TelemetrySample>,
    is_valid: bool,
}

Driver { code, name, number, team, team_color }
DriverSession { driver, laps }
Session { year, circuit, session_type, date, drivers }
```

### Turn Model (`turn.rs`)

Turns are defined by bounding boxes in the X/Z plane:

```rust
BoundingBox { min_x, max_x, min_z, max_z }

Turn {
    name: String,           // e.g., "T1", "Maggots"
    bounds: BoundingBox,    // Overall turn area
    approach: BoundingBox,  // Entry segment
    apex: BoundingBox,      // Middle segment
    exit: BoundingBox,      // Exit segment
}
```

Methods:
- `turn.filter_samples(&samples)` → samples within overall bounds
- `turn.filter_segment(&samples, TurnSegment::Apex)` → segment-specific
- `turn.classify_position(pos)` → which segment a point is in

### Analysis (`analysis.rs`)

```rust
TurnAnalysis::analyze(lap, turn) → {
    lap_number,
    total_time: SegmentTime,
    approach: SegmentTime,
    apex: SegmentTime,
    exit: SegmentTime,
}

SegmentTime { time, entry_speed, min_speed, exit_speed }

TurnComparison::compare(laps, turn) → {
    turn_name,
    analyses: Vec<TurnAnalysis>,  // Sorted by total time (fastest first)
    uniform_fastest: bool,         // True if overall fastest = fastest in all segments
    segment_bests: SegmentBests,   // Which lap was fastest in each segment
}
```

The `uniform_fastest` flag is key: when false, the UI should offer subsegment analysis because different laps may be faster in different parts of the turn.

---

## Crate: f1-app

Bevy application for visualization.

### State Machine (`resources.rs`)

```rust
AppState {
    Loading,        // Initial state
    SessionSelect,  // Choose session file
    Replay,         // Playback mode
    Analysis,       // Turn analysis mode
}
```

### Resources

| Resource | Purpose |
|----------|---------|
| `SessionData` | Loaded `Session` from JSON |
| `ReplayState` | `current_time`, `speed`, `playing`, `selected_drivers` |
| `TurnData` | Circuit turn definitions |
| `SelectedTurn` | Currently analyzed turn |
| `ComparisonDrivers` | Drivers being compared |

### Components

| Component | Purpose |
|-----------|---------|
| `Car` | Marker + driver code, current lap/sample |
| `Trail` | Position history for line rendering |
| `MainCamera` | Camera marker |
| `TurnBoundary` | Visualization of turn bounds |
| `TrackEntity` | Track visualization elements |

### Plugins

| Plugin | Responsibility |
|--------|----------------|
| `CameraPlugin` | 2D camera setup |
| `ReplayPlugin` | Time advancement, car position updates |
| `UiPlugin` | UI panels, controls, telemetry display |

### Systems

| System | When | Purpose |
|--------|------|---------|
| `update_replay` | `Replay` state | Advance time, update positions |
| `handle_playback_input` | (not wired) | Space=play/pause, brackets=speed |
| `update_trails` | (not wired) | Record car positions for trail |
| `draw_trails` | (not wired) | Render trails with gizmos |

---

## Data Pipeline

```
┌─────────────┐     ┌──────────────┐     ┌───────────┐     ┌─────────────┐
│   F1 API    │────▶│   fastf1    │────▶│   JSON    │────▶│  f1-data    │
│  (timing)   │     │  (Python)   │     │  export   │     │  loader     │
└─────────────┘     └──────────────┘     └───────────┘     └─────────────┘
                                                                  │
                                                                  ▼
                                              ┌───────────────────────────┐
                                              │     Bevy Resources        │
                                              │  SessionData, ReplayState │
                                              └───────────────────────────┘
                                                          │
                                                          ▼
                                              ┌───────────────────────────┐
                                              │     ECS Systems           │
                                              │  update_replay, draw_*    │
                                              └───────────────────────────┘
```

### JSON Format

The Python export script produces JSON matching `Session` struct:

```json
{
  "year": 2024,
  "circuit": "Monaco Grand Prix",
  "circuit_short": "Monaco",
  "session_type": "Qualifying",
  "date": "2024-05-25T14:00:00Z",
  "drivers": [
    {
      "driver": {
        "code": "VER",
        "name": "Max Verstappen",
        "number": 1,
        "team": "Red Bull Racing",
        "team_color": "#3671C6"
      },
      "laps": [
        {
          "number": 1,
          "lap_time": 72.345,
          "sector_times": [23.456, 24.567, 24.322],
          "samples": [
            {
              "position": [1234.5, 0.0, 567.8],
              "time": 0.0,
              "throttle": 100,
              "brake": 0,
              "gear": 7,
              "speed": 285,
              "drs": "Off"
            }
          ],
          "is_valid": true
        }
      ]
    }
  ]
}
```

---

## Coordinate System

F1 telemetry uses:
- **X**: Horizontal track axis
- **Y**: Forward track axis (swapped to Z for Bevy)
- **Z**: Elevation (swapped to Y for Bevy)

The Python export script handles this swap: `[X, Z, Y]` in output.

Bevy 2D uses X/Y, so for top-down view we use X and Z from the data (ignoring elevation for 2D).

---

## Status

> **This is the single source of truth for project status.**
> Update this section when completing work. Do not track status elsewhere.

### Completed
- [x] Workspace structure (f1-data, f1-app crates)
- [x] Core types: `TelemetrySample`, `Lap`, `Driver`, `Session`
- [x] Turn model with approach/apex/exit segments
- [x] Analysis: `TurnAnalysis`, `TurnComparison`
- [x] JSON loader
- [x] Python data export script (fastf1, uv-managed)
- [x] Bevy app scaffold with plugins
- [x] ECS components and resources defined
- [x] Fetched 2025 US GP Austin data (Q: 20 drivers/285 laps, R: 20 drivers/1067 laps)

### In Progress
_(Nothing currently in progress)_

### Up Next
- [ ] Load session JSON on startup
- [ ] Render track outline from position data
- [ ] Spawn car entities for selected drivers
- [ ] Interpolate positions based on replay time
- [ ] Playback controls (play/pause, speed)

### Backlog

**Comparison View**
- [ ] Multi-car display with team colors
- [ ] Delta time display between drivers
- [ ] Ghost car mode (compare to reference lap)

**Turn Analysis**
- [ ] Turn bounding box creation UI
- [ ] Save/load turn definitions per circuit
- [ ] Turn time table (sorted by segment)
- [ ] Highlight segment bests when not uniform

**Polish**
- [ ] Telemetry graphs (speed, throttle, brake)
- [ ] 3D view option
- [ ] Session selection UI
- [ ] Export analysis results

---

## Development Notes

### Why Workspace?

- `f1-data` can be tested without spinning up Bevy
- Potential for CLI analysis tool later
- Clear dependency boundaries (f1-data has no bevy dep)

### Why Python for Data?

FastF1 has years of development handling:
- F1 timing API quirks
- Data normalization
- Caching
- Error recovery

Reimplementing this in Rust would be significant scope creep with little benefit.

### Bevy Version

Using Bevy 0.15. Key patterns:
- `init_state::<AppState>()` for state machine
- `run_if(in_state(AppState::Replay))` for conditional systems
- Resources for shared data, Components for per-entity data
