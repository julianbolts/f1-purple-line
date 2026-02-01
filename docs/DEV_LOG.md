# F1 Purple Line - Development Log

Narrative history of development sessions. Newest entries at the top.

> **For current project status, see `PROJECT.md` → "Status" section.**
> This log is for historical context only—do not track status here.

---

## 2025-02-01: Project Initialization

### Summary

Set up the initial project structure as a Rust workspace with two crates: `f1-data` (core library) and `f1-app` (Bevy application).

### What Was Built

**f1-data crate:**
- `types.rs` — Core data structures (`TelemetrySample`, `Lap`, `Driver`, `Session`)
- `loader.rs` — JSON deserialization with error handling
- `turn.rs` — Turn bounding box model with approach/apex/exit segments
- `analysis.rs` — `TurnAnalysis` and `TurnComparison` for lap analysis

**f1-app crate:**
- Basic Bevy app with window configuration
- State machine: `Loading → SessionSelect → Replay → Analysis`
- Plugin structure: `CameraPlugin`, `ReplayPlugin`, `UiPlugin`
- ECS components: `Car`, `Trail`, `MainCamera`, `TurnBoundary`
- Resources: `SessionData`, `ReplayState`, `TurnData`
- System stubs: input handling, replay update, trail rendering

**Tooling:**
- `tools/fetch_session.py` — Export F1 data via fastf1 to JSON
- Handles coordinate system swap (F1 Y/Z → Bevy Z/Y)

**Documentation:**
- `CLAUDE.md` — Quick-start for AI agents
- `docs/PROJECT.md` — Architecture and status tracking
- `docs/DEV_LOG.md` — This file

### Key Decisions

| Decision | Rationale |
|----------|-----------|
| 2 crates, not 3 | A separate "fastf1 wrapper" crate would be overengineering |
| Python for data export | FastF1 has years of F1 API handling; no need to reimplement |
| Bounding boxes for turns | Simple model; polygons could be added later if needed |
| 3-segment turns | Approach/apex/exit matches driver mental model |
| `uniform_fastest` flag | Avoids redundant UI when one lap is fastest everywhere |

### Discussion

**User requirement:** Analyze turns with approach/apex/exit segments. Show subsegment bests only when the overall fastest lap isn't fastest in all segments.

**Implementation:** `TurnComparison` computes overall ranking and per-segment bests. The `uniform_fastest: bool` flag tells the UI whether to expose subsegment sorting.

### Build Status

`cargo check` passes. Warnings for unused code are expected (stubs not yet wired).

---

## Entry Template

```markdown
## YYYY-MM-DD: Title

### Summary
One paragraph overview.

### What Was Built
- Bullet points of completed work

### Key Decisions
| Decision | Rationale |
|----------|-----------|

### Issues Encountered
- Problem → Solution

### Discussion
Context, tradeoffs, or notable implementation details.

### Build Status
`cargo check` / `cargo test` results.
```
