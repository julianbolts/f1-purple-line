//! Rendering systems for cars, trails, and track.

use bevy::prelude::*;

use crate::components::{Car, Trail};
use crate::resources::SessionData;

/// Update car trail positions.
pub fn update_trails(mut query: Query<(&Transform, &mut Trail), With<Car>>) {
    for (transform, mut trail) in &mut query {
        trail.push(transform.translation);
    }
}

/// Draw car trails using gizmos.
pub fn draw_trails(query: Query<&Trail, With<Car>>, mut gizmos: Gizmos) {
    for trail in &query {
        if trail.positions.len() < 2 {
            continue;
        }

        // Draw trail as connected line segments
        for window in trail.positions.windows(2) {
            gizmos.line_2d(
                window[0].truncate(),
                window[1].truncate(),
                Color::srgba(1.0, 1.0, 1.0, 0.5),
            );
        }
    }
}

/// Draw the circuit track map using gizmos.
pub fn draw_circuit(session_data: Res<SessionData>, mut gizmos: Gizmos) {
    let Some(session) = &session_data.session else {
        return;
    };

    // Find a reference lap (first driver, first valid lap)
    // In a real app we might want a dedicated map path, but this works for now
    let reference_lap = session
        .drivers
        .first()
        .and_then(|d| d.laps.iter().find(|l| !l.samples.is_empty()));

    if let Some(lap) = reference_lap {
        // Calculate bounds to center and scale the track
        // F1 coordinates: X, Z (horizontal plane), Y (elevation)
        // Bevy 2D: X (right), Y (up)
        // Transformation: F1 X -> Bevy X, F1 Z -> Bevy Y
        let points: Vec<Vec2> = lap
            .samples
            .iter()
            .map(|s| Vec2::new(s.position.x, s.position.z)) // Use Z as Y for 2D top-down
            .collect();

        if points.is_empty() {
            return;
        }

        let min_x = points.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
        let max_x = points.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
        let min_y = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
        let max_y = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);

        let width = max_x - min_x;
        let height = max_y - min_y;
        let center = Vec2::new(min_x + width / 2.0, min_y + height / 2.0);

        // Scale to fit window (approx 800px height for 1080p, simplify to fixed scale for now or auto-fit)
        // Let's assume a desired drawing area of ~800x800
        let scale = 800.0 / width.max(height);

        for window in points.windows(2) {
            let p1 = (window[0] - center) * scale;
            let p2 = (window[1] - center) * scale;
            
            gizmos.line_2d(p1, p2, Color::WHITE);
        }
    }
}
