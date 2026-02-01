# F1 Purple Line - AI Agent Context

## Quick Start

```bash
cargo run                     # Build and run
cargo test -p f1-data         # Test core library
```

To fetch sample data (requires Python + fastf1):
```bash
cd tools && pip install -r requirements.txt
python fetch_session.py --year 2024 --circuit Monaco --session Q
```

## Status

**Check `docs/PROJECT.md` → "Status" section for current project state.**

This is the single source of truth. Update it when completing work.

## Project Structure

```
crates/
├── f1-data/     # Core library: types, loading, analysis (no Bevy dep)
└── f1-app/      # Bevy application: visualization, UI, replay
tools/           # Python scripts for data fetching via fastf1
data/            # Session JSON files (gitignored cache in .cache/)
docs/
├── PROJECT.md   # Architecture + STATUS (single source of truth)
└── DEV_LOG.md   # Development history (narrative, not status)
```

## Key Files

| File | Purpose |
|------|---------|
| `crates/f1-data/src/types.rs` | `TelemetrySample`, `Lap`, `Driver`, `Session` |
| `crates/f1-data/src/turn.rs` | `Turn` with approach/apex/exit segments |
| `crates/f1-data/src/analysis.rs` | `TurnAnalysis`, `TurnComparison` |
| `crates/f1-app/src/resources.rs` | Bevy resources: `SessionData`, `ReplayState` |
| `crates/f1-app/src/components.rs` | ECS components: `Car`, `Trail` |
| `crates/f1-app/src/plugins/` | Bevy plugins: camera, replay, ui |

## Data Flow

```
fastf1 (Python) → JSON export → f1-data loader → Bevy resources → ECS systems
```

## Conventions

- Bevy 0.15 patterns: `init_state`, `run_if(in_state(...))`
- Coordinates: X/Z = track plane, Y = elevation
- Driver codes: 3-letter (e.g., "VER", "HAM")
- Times: seconds (f64), speeds: km/h (u16)

## Architecture Decisions

1. **Workspace split**: `f1-data` has no Bevy dependency (testable, reusable)
2. **Python for data**: FastF1 handles F1 API; we consume exported JSON
3. **Turn segments**: 3 bounding boxes (approach/apex/exit) per turn
4. **Comparison logic**: `TurnComparison.uniform_fastest` flags when subsegment analysis is useful

## See Also

- `docs/PROJECT.md` - Full architecture + **status tracking**
- `docs/DEV_LOG.md` - Development history
- [f1-race-replay](https://github.com/IAmTomShaw/f1-race-replay) - Python inspiration
- [fastf1 docs](https://docs.fastf1.dev/) - Data source
