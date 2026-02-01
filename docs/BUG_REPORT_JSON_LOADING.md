# Bug Report: JSON Session Loading Failure

**Severity**: Critical (Blocks core functionality)
**Date**: 2025-10-18 (Project Time)
**Component**: `f1-data` (JSON Loader)

## Description
The application fails to load valid session JSON files located in the `data/` directory. When attempting to load a session (e.g., `2025_austin_q.json`), the application crashes or logs an error.

## Error Log
```
ERROR f1_purple_line::plugins::replay: Failed to load session: Json(Error("premature end of input", line: 6, column: 31))
```

## Investigation
The error points to Line 6 of the JSON file:
```json
  "date": "2025-10-18T21:00:00",
```
This line ends at column 32 (including the comma). The error "premature end of input" at column 31 suggests the parser expected more characters within the string but reached the closing quote.

### Root Cause Analysis
1. **Rust Type**: The `Session` struct in `crates/f1-data/src/types.rs` defines `date` as:
   ```rust
   pub date: DateTime<Utc>,
   ```
2. **Serialization Expectation**: `chrono::DateTime<Utc>`'s default deserialization expects an RFC 3339 / ISO 8601 string **with timezone information** (e.g., `"2025-10-18T21:00:00Z"` or `"2025-10-18T21:00:00+00:00"`).
3. **Data Mismatch**: The JSON file contains a "Naive" datetime string (`"2025-10-18T21:00:00"`) without a timezone suffix.
4. **Parser Failure**: The `serde` deserializer consumes the timestamp digits and expects a 'Z' or offset. Finding the closing quote `"` instead, it reports "premature end of input".

## Reproduction Steps
1. Run the application: `cargo run`
2. Select "Austin 2025 Q"
3. Observe error in terminal.

## Proposed Solutions

### Option 1: Fix Data (Recommended if controlling data source)
Modify the Python export scripts (`tools/fetch_session.py`) to ensure dates are exported with timezone information (UTC).

### Option 2: Fix Rust Code (Resilient approach)
Modify `crates/f1-data/src/types.rs` to handle naive dates by assuming UTC.

**Change:**
```rust
use chrono::NaiveDateTime;

// ...

pub struct Session {
    // ...
    // Change field type or add custom deserialization
    pub date: DateTime<Utc>, 
}
```

Or clearer:
Change the field to `NaiveDateTime` if timezone is implied, or use a custom deserializer function `deserialize_naive_as_utc`.
