//! Advanced shape generators for svg-rs
//!
//! This module provides generators for complex shapes like stars, regular polygons (ngons),
//! and other geometric shapes, inspired by svg.shapes.js.

use std::f64::consts::PI;

/// Configuration for star shapes
#[derive(Debug, Clone)]
pub struct StarConfig {
    /// Number of star spikes/points
    pub spikes: u32,
    /// Inner radius (distance from center to inner points)
    pub inner: f64,
    /// Outer radius (distance from center to outer points)
    pub outer: f64,
}

impl Default for StarConfig {
    fn default() -> Self {
        Self {
            spikes: 5,
            inner: 50.0,
            outer: 100.0,
        }
    }
}

/// Configuration for regular polygon (ngon) shapes
#[derive(Debug, Clone)]
pub struct NgonConfig {
    /// Number of edges/sides
    pub edges: u32,
    /// Radius from center to vertices
    pub radius: f64,
}

impl Default for NgonConfig {
    fn default() -> Self {
        Self {
            edges: 6,
            radius: 100.0,
        }
    }
}

/// Generate points for a star shape
pub fn star_points(config: &StarConfig) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let degrees = 360.0 / config.spikes as f64;
    
    for i in 0..config.spikes {
        let angle = i as f64 * degrees - 90.0;
        
        // Outer point
        let outer_x = config.outer * (angle * PI / 180.0).cos();
        let outer_y = config.outer * (angle * PI / 180.0).sin();
        points.push((outer_x, outer_y));
        
        // Inner point
        let inner_angle = angle + degrees / 2.0;
        let inner_x = config.inner * (inner_angle * PI / 180.0).cos();
        let inner_y = config.inner * (inner_angle * PI / 180.0).sin();
        points.push((inner_x, inner_y));
    }
    
    points
}

/// Generate points for a regular polygon (ngon)
pub fn ngon_points(config: &NgonConfig) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let degrees = 360.0 / config.edges as f64;
    
    for i in 0..config.edges {
        let angle = i as f64 * degrees - 90.0;
        let x = config.radius * (angle * PI / 180.0).cos();
        let y = config.radius * (angle * PI / 180.0).sin();
        points.push((x, y));
    }
    
    points
}

/// Convert points to SVG polygon points string
pub fn points_to_string(points: &[(f64, f64)]) -> String {
    points
        .iter()
        .map(|(x, y)| format!("{:.2},{:.2}", x, y))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Configuration for cross shapes
#[derive(Debug, Clone)]
pub struct CrossConfig {
    /// Width of the cross
    pub width: f64,
    /// Height of the cross
    pub height: f64,
    /// Thickness of the cross arms
    pub thickness: f64,
}

impl Default for CrossConfig {
    fn default() -> Self {
        Self {
            width: 100.0,
            height: 100.0,
            thickness: 20.0,
        }
    }
}

/// Generate points for a cross shape
pub fn cross_points(config: &CrossConfig) -> Vec<(f64, f64)> {
    let w = config.width / 2.0;
    let h = config.height / 2.0;
    let t = config.thickness / 2.0;
    
    vec![
        (-t, -h),   // top left of vertical bar
        (t, -h),    // top right of vertical bar
        (t, -t),    // inner top right
        (w, -t),    // outer top right
        (w, t),     // outer bottom right
        (t, t),     // inner bottom right
        (t, h),     // bottom right of vertical bar
        (-t, h),    // bottom left of vertical bar
        (-t, t),    // inner bottom left
        (-w, t),    // outer bottom left
        (-w, -t),   // outer top left
        (-t, -t),   // inner top left
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_points() {
        let config = StarConfig {
            spikes: 5,
            inner: 30.0,
            outer: 60.0,
        };
        let points = star_points(&config);
        assert_eq!(points.len(), 10); // 5 spikes * 2 points each
    }

    #[test]
    fn test_ngon_points() {
        let config = NgonConfig {
            edges: 6,
            radius: 50.0,
        };
        let points = ngon_points(&config);
        assert_eq!(points.len(), 6);
    }

    #[test]
    fn test_points_to_string() {
        let points = vec![(0.0, 0.0), (10.0, 20.0), (30.0, 40.0)];
        let result = points_to_string(&points);
        assert_eq!(result, "0.00,0.00 10.00,20.00 30.00,40.00");
    }

    #[test]
    fn test_cross_points() {
        let config = CrossConfig {
            width: 60.0,
            height: 80.0,
            thickness: 20.0,
        };
        let points = cross_points(&config);
        assert_eq!(points.len(), 12); // Cross has 12 points
    }
}
