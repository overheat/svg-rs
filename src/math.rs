use std::f64::consts::PI;

/// Mathematical utilities for SVG operations
pub struct Math;

impl Math {
    /// Calculate angle between two points or three points
    pub fn angle(p1: &Point, p2: &Point, p3: Option<&Point>) -> f64 {
        match p3 {
            Some(p3) => (Self::angle(p1, p3, None) - Self::angle(p2, p3, None)).abs(),
            None => {
                let mut angle = (p2.y - p1.y).atan2(p2.x - p1.x);
                while angle < 0.0 {
                    angle += 2.0 * PI;
                }
                angle
            }
        }
    }

    /// Convert degrees to radians
    pub fn rad(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    /// Convert radians to degrees
    pub fn deg(radians: f64) -> f64 {
        radians * 180.0 / PI
    }

    /// Snap angle to nearest direction
    pub fn snap_to_angle(angle: f64, directions: &[f64]) -> f64 {
        let mut dirs = directions.to_vec();
        dirs.push(directions.iter().fold(f64::INFINITY, |a, &b| a.min(b)) + 2.0 * PI);
        
        let mut angle = angle;
        while angle > 2.0 * PI {
            angle -= 2.0 * PI;
        }
        while angle < 0.0 {
            angle += 2.0 * PI;
        }

        let mut min_diff = 100.0;
        for (i, &dir) in dirs.iter().enumerate() {
            let diff = (angle - dir).abs();
            if diff > min_diff {
                return dirs[i - 1];
            }
            min_diff = diff;
        }
        dirs[0]
    }

    /// Linear interpolation
    pub fn lerp(a: f64, b: f64, x: f64) -> f64 {
        a + x * (b - a)
    }
}

/// 2D Point structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Distance to another point
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Angle to another point
    pub fn angle_to(&self, other: &Point) -> f64 {
        Math::angle(self, other, None)
    }
}

/// Line structure for geometric calculations
#[derive(Debug, Clone)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        let mut line = Self {
            p1,
            p2,
            a: 0.0,
            b: 0.0,
            c: 0.0,
        };
        line.update(p1, p2);
        line
    }

    pub fn update(&mut self, p1: Point, p2: Point) {
        self.p1 = p1;
        self.p2 = p2;
        self.a = p2.y - p1.y;
        self.b = p1.x - p2.x;
        self.c = p1.x * p2.y - p2.x * p1.y;
    }

    /// Check if parallel to another line
    pub fn parallel(&self, other: &Line) -> bool {
        (self.a * other.b - other.a * self.b).abs() < f64::EPSILON
    }

    /// Move point along line by distance
    pub fn move_point(&self, from: &Point, towards: &Point, distance: f64) -> Point {
        let sign = if from.x > towards.x {
            -1.0
        } else if from.x < towards.x {
            1.0
        } else if from.y > towards.y {
            -1.0
        } else {
            1.0
        };

        let theta = ((self.p1.y - self.p2.y).abs() / (self.p1.x - self.p2.x).abs()).atan();
        let dy = distance * theta.sin();
        let dx = distance * theta.cos();

        Point::new(from.x + sign * dx, from.y + sign * dy)
    }

    /// Find intersection with another line
    pub fn intersection(&self, other: &Line) -> LineIntersection {
        let det = self.a * other.b - other.a * self.b;
        if det.abs() < f64::EPSILON {
            LineIntersection::Parallel
        } else {
            LineIntersection::Point(Point::new(
                (other.b * self.c - self.b * other.c) / det,
                (self.a * other.c - other.a * self.c) / det,
            ))
        }
    }

    /// Get midpoint of line segment
    pub fn midpoint(&self) -> Point {
        self.interpolated_point(0.5)
    }

    /// Get squared length of line segment
    pub fn segment_length_squared(&self) -> f64 {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        dx * dx + dy * dy
    }

    /// Find closest linear interpolation parameter for point
    pub fn closest_linear_interpolation(&self, p: &Point) -> f64 {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        ((p.x - self.p1.x) * dx + (p.y - self.p1.y) * dy) / self.segment_length_squared()
    }

    /// Get interpolated point at parameter t (0.0 to 1.0)
    pub fn interpolated_point(&self, t: f64) -> Point {
        Point::new(
            Math::lerp(self.p1.x, self.p2.x, t),
            Math::lerp(self.p1.y, self.p2.y, t),
        )
    }

    /// Find closest point on line to given point
    pub fn closest_point(&self, p: &Point) -> Point {
        self.interpolated_point(self.closest_linear_interpolation(p))
    }

    /// Create perpendicular line through point with given distance
    pub fn perpendicular_line(&self, p: &Point, distance: f64) -> Line {
        let dx = self.p1.x - self.p2.x;
        let dy = self.p1.y - self.p2.y;
        let dist = (dx * dx + dy * dy).sqrt();
        let dx = dx / dist;
        let dy = dy / dist;

        Line::new(
            Point::new(p.x + distance * dy, p.y - distance * dx),
            Point::new(p.x - distance * dy, p.y + distance * dx),
        )
    }
}

/// Result of line intersection calculation
#[derive(Debug, Clone)]
pub enum LineIntersection {
    Point(Point),
    Parallel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_calculation() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(1.0, 1.0);
        let angle = Math::angle(&p1, &p2, None);
        assert!((angle - PI / 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(Math::lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(Math::lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(Math::lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_line_midpoint() {
        let line = Line::new(Point::new(0.0, 0.0), Point::new(10.0, 10.0));
        let mid = line.midpoint();
        assert_eq!(mid, Point::new(5.0, 5.0));
    }
}
