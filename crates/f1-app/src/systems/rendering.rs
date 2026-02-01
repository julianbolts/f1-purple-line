//! Rendering systems for cars, trails, and track.

use bevy::prelude::*;

use crate::components::{Car, Trail};

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
