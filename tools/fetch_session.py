#!/usr/bin/env python3
"""
Fetch F1 session data using fastf1 and export to JSON for the Rust application.

Usage (with uv):
    uv run fetch_session.py --year 2025 --circuit Austin --session Q
    uv run fetch_session.py --year 2025 --circuit Austin --session R

Dependencies managed via pyproject.toml (fastf1, pandas).
"""

import argparse
import json
from datetime import datetime
from pathlib import Path

import fastf1


def fetch_session(year: int, circuit: str, session_type: str, output_dir: Path):
    """Fetch session data and export to JSON."""

    # Enable cache for faster subsequent loads
    cache_dir = output_dir / ".cache"
    cache_dir.mkdir(parents=True, exist_ok=True)
    fastf1.Cache.enable_cache(str(cache_dir))

    # Map session type shorthand
    session_map = {
        "FP1": "FP1",
        "FP2": "FP2",
        "FP3": "FP3",
        "Q": "Q",
        "Sprint": "Sprint",
        "R": "R",
        "Race": "R",
    }

    session_key = session_map.get(session_type, session_type)

    print(f"Loading {year} {circuit} {session_key}...")
    session = fastf1.get_session(year, circuit, session_key)
    session.load(telemetry=True, weather=False, messages=False)

    drivers_data = []

    for driver in session.drivers:
        driver_info = session.get_driver(driver)
        driver_laps = session.laps.pick_drivers(driver)

        laps_data = []
        for _, lap in driver_laps.iterrows():
            # Get telemetry for this lap
            try:
                telemetry = lap.get_telemetry()
            except Exception:
                continue

            if telemetry.empty:
                continue

            samples = []
            for _, t in telemetry.iterrows():
                samples.append({
                    "position": [float(t["X"]), float(t.get("Z", 0)), float(t["Y"])],  # Swap Y/Z for Bevy coords
                    "time": float(t["SessionTime"].total_seconds()) if hasattr(t["SessionTime"], "total_seconds") else float(t["SessionTime"]),
                    "throttle": int(t["Throttle"]),
                    "brake": int(t["Brake"]),
                    "gear": int(t["nGear"]),
                    "speed": int(t["Speed"]),
                    "drs": "Active" if t.get("DRS", 0) > 10 else "Off",
                })

            sector_times = [
                float(lap["Sector1Time"].total_seconds()) if pd.notna(lap["Sector1Time"]) else None,
                float(lap["Sector2Time"].total_seconds()) if pd.notna(lap["Sector2Time"]) else None,
                float(lap["Sector3Time"].total_seconds()) if pd.notna(lap["Sector3Time"]) else None,
            ]

            laps_data.append({
                "number": int(lap["LapNumber"]),
                "lap_time": float(lap["LapTime"].total_seconds()) if pd.notna(lap["LapTime"]) else None,
                "sector_times": sector_times,
                "samples": samples,
                "is_valid": bool(lap["IsAccurate"]) if pd.notna(lap["IsAccurate"]) else True,
            })

        if not laps_data:
            continue

        drivers_data.append({
            "driver": {
                "code": str(driver_info["Abbreviation"]),
                "name": f"{driver_info['FirstName']} {driver_info['LastName']}",
                "number": int(driver_info["DriverNumber"]),
                "team": str(driver_info["TeamName"]),
                "team_color": f"#{driver_info['TeamColor']}" if driver_info["TeamColor"] else "#FFFFFF",
            },
            "laps": laps_data,
        })

    # Build session output
    session_data = {
        "year": year,
        "circuit": session.event["EventName"],
        "circuit_short": session.event["Location"],
        "session_type": {
            "FP1": "Practice1",
            "FP2": "Practice2",
            "FP3": "Practice3",
            "Q": "Qualifying",
            "Sprint": "Sprint",
            "R": "Race",
        }.get(session_key, "Race"),
        "date": session.date.isoformat() if session.date else datetime.now().isoformat(),
        "drivers": drivers_data,
    }

    # Write output
    output_file = output_dir / f"{year}_{circuit.lower().replace(' ', '_')}_{session_key.lower()}.json"
    output_file.parent.mkdir(parents=True, exist_ok=True)

    with open(output_file, "w") as f:
        json.dump(session_data, f, indent=2)

    print(f"Wrote {output_file}")
    print(f"  {len(drivers_data)} drivers, {sum(len(d['laps']) for d in drivers_data)} total laps")


if __name__ == "__main__":
    import pandas as pd  # Import here to fail fast if not installed

    parser = argparse.ArgumentParser(description="Fetch F1 session data")
    parser.add_argument("--year", type=int, required=True, help="Season year (e.g., 2024)")
    parser.add_argument("--circuit", type=str, required=True, help="Circuit name (e.g., Monaco, Silverstone)")
    parser.add_argument("--session", type=str, required=True, help="Session type: FP1, FP2, FP3, Q, Sprint, R")
    parser.add_argument("--output", type=str, default="data", help="Output directory")

    args = parser.parse_args()

    fetch_session(args.year, args.circuit, args.session, Path(args.output))
