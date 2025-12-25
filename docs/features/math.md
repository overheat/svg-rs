# Math Feature

The math feature provides geometric calculations and utilities for SVG operations.

## Enabling the Feature

```toml
[dependencies]
svg-rs = { version = "0.2", features = ["math"] }
```

```rust
use svg_rs::*;
use svg_rs::math::*;
```

## Core Components

### Math Utilities

The `Math` struct provides static methods for common calculations:

#### Angle Calculations

```rust
let p1 = Point::new(0.0, 0.0);
let p2 = Point::new(100.0, 100.0);

// Angle between two points (in radians)
let angle = Math::angle(&p1, &p2, None);
println!("Angle: {} radians", angle);

// Angle between three points
let p3 = Point::new(200.0, 0.0);
let angle_between = Math::angle(&p1, &p2, Some(&p3));
```

#### Unit Conversions

```rust
// Convert degrees to radians
let radians = Math::rad(45.0);  // π/4

// Convert radians to degrees
let degrees = Math::deg(std::f64::consts::PI / 4.0);  // 45.0
```

#### Linear Interpolation

```rust
// Interpolate between two values
let result = Math::lerp(0.0, 100.0, 0.5);  // 50.0
let result = Math::lerp(10.0, 20.0, 0.25); // 12.5
```

#### Angle Snapping

```rust
// Snap angle to nearest direction
let angle = Math::rad(47.0);  // 47 degrees
let directions = [0.0, Math::rad(45.0), Math::rad(90.0), Math::rad(135.0)];
let snapped = Math::snap_to_angle(angle, &directions);
println!("Snapped to: {} degrees", Math::deg(snapped));  // 45 degrees
```

### Point Structure

The `Point` struct represents 2D coordinates:

```rust
// Create points
let p1 = Point::new(10.0, 20.0);
let p2 = Point::new(30.0, 40.0);

// Calculate distance
let distance = p1.distance_to(&p2);
println!("Distance: {}", distance);

// Calculate angle to another point
let angle = p1.angle_to(&p2);
```

### Line Structure

The `Line` struct represents line segments and provides geometric operations:

#### Creating Lines

```rust
let p1 = Point::new(0.0, 0.0);
let p2 = Point::new(100.0, 100.0);
let line = Line::new(p1, p2);
```

#### Line Properties

```rust
// Get midpoint
let midpoint = line.midpoint();

// Get line length (squared for performance)
let length_sq = line.segment_length_squared();

// Check if lines are parallel
let line2 = Line::new(Point::new(10.0, 10.0), Point::new(110.0, 110.0));
let is_parallel = line.parallel(&line2);
```

#### Point Interpolation

```rust
// Get point at parameter t (0.0 to 1.0)
let quarter_point = line.interpolated_point(0.25);
let half_point = line.interpolated_point(0.5);
let three_quarter_point = line.interpolated_point(0.75);

// Find closest point on line to given point
let test_point = Point::new(50.0, 30.0);
let closest = line.closest_point(&test_point);
```

#### Perpendicular Lines

```rust
// Create perpendicular line through a point
let midpoint = line.midpoint();
let perp_line = line.perpendicular_line(&midpoint, 50.0);
```

#### Line Intersections

```rust
let line1 = Line::new(Point::new(0.0, 0.0), Point::new(100.0, 100.0));
let line2 = Line::new(Point::new(0.0, 100.0), Point::new(100.0, 0.0));

match line1.intersection(&line2) {
    LineIntersection::Point(p) => {
        println!("Lines intersect at: ({}, {})", p.x, p.y);
    }
    LineIntersection::Parallel => {
        println!("Lines are parallel");
    }
}
```

## Complete Example

```rust
use svg_rs::*;
use svg_rs::math::*;

fn main() {
    let mut canvas = Svg::new(800, 600);
    
    // Example 1: Basic angle calculation and visualization
    let p1 = Point::new(100.0, 100.0);
    let p2 = Point::new(200.0, 150.0);
    let angle = Math::angle(&p1, &p2, None);
    
    println!("Angle between points: {} radians ({} degrees)", 
             angle, Math::deg(angle));
    
    // Draw the points and line
    canvas.circle(5).fill("#f06").center(p1.x as i32, p1.y as i32);
    canvas.circle(5).fill("#0f6").center(p2.x as i32, p2.y as i32);
    canvas.line(p1.x as i32, p1.y as i32, p2.x as i32, p2.y as i32)
        .stroke("#333").stroke_width(2);
    
    // Example 2: Line operations
    let line = Line::new(Point::new(50.0, 200.0), Point::new(250.0, 300.0));
    let midpoint = line.midpoint();
    
    // Draw the line and its midpoint
    canvas.line(50, 200, 250, 300).stroke("#666").stroke_width(2);
    canvas.circle(5).fill("#f60").center(midpoint.x as i32, midpoint.y as i32);
    
    // Example 3: Perpendicular line
    let perp_line = line.perpendicular_line(&midpoint, 50.0);
    canvas.line(
        perp_line.p1.x as i32, perp_line.p1.y as i32,
        perp_line.p2.x as i32, perp_line.p2.y as i32
    ).stroke("#06f").stroke_width(2);
    
    // Example 4: Linear interpolation visualization
    for i in 0..=10 {
        let t = i as f64 / 10.0;
        let point = line.interpolated_point(t);
        canvas.circle(3).fill("#0f6")
            .center(point.x as i32, point.y as i32);
    }
    
    // Example 5: Line intersection
    let line1 = Line::new(Point::new(400.0, 100.0), Point::new(600.0, 300.0));
    let line2 = Line::new(Point::new(400.0, 300.0), Point::new(600.0, 100.0));
    
    // Draw the lines
    canvas.line(400, 100, 600, 300).stroke("#e74c3c").stroke_width(2);
    canvas.line(400, 300, 600, 100).stroke("#2ecc71").stroke_width(2);
    
    // Find and mark intersection
    if let LineIntersection::Point(intersection) = line1.intersection(&line2) {
        canvas.circle(8).fill("#f39c12")
            .center(intersection.x as i32, intersection.y as i32);
    }
    
    // Example 6: Angle snapping demonstration
    let center = Point::new(400.0, 450.0);
    let radius = 80.0;
    
    // Draw compass directions
    let directions = [0.0, Math::rad(45.0), Math::rad(90.0), Math::rad(135.0), 
                     Math::rad(180.0), Math::rad(225.0), Math::rad(270.0), Math::rad(315.0)];
    
    for &dir in &directions {
        let end_x = center.x + radius * dir.cos();
        let end_y = center.y + radius * dir.sin();
        canvas.line(center.x as i32, center.y as i32, end_x as i32, end_y as i32)
            .stroke("#ddd").stroke_width(1);
    }
    
    // Test angle and its snapped version
    let test_angle = Math::rad(67.0);
    let snapped_angle = Math::snap_to_angle(test_angle, &directions);
    
    // Draw test angle (red)
    let test_end_x = center.x + radius * test_angle.cos();
    let test_end_y = center.y + radius * test_angle.sin();
    canvas.line(center.x as i32, center.y as i32, test_end_x as i32, test_end_y as i32)
        .stroke("#e74c3c").stroke_width(3);
    
    // Draw snapped angle (green)
    let snap_end_x = center.x + radius * snapped_angle.cos();
    let snap_end_y = center.y + radius * snapped_angle.sin();
    canvas.line(center.x as i32, center.y as i32, snap_end_x as i32, snap_end_y as i32)
        .stroke("#2ecc71").stroke_width(3);
    
    // Add labels
    canvas.text("Angle calculation").move_to(20, 120).font_size(14).fill("#666");
    canvas.text("Line midpoint & perpendicular").move_to(20, 220).font_size(14).fill("#666");
    canvas.text("Linear interpolation points").move_to(20, 320).font_size(14).fill("#666");
    canvas.text("Line intersection").move_to(400, 80).font_size(14).fill("#666");
    canvas.text("Angle snapping").move_to(320, 380).font_size(14).fill("#666");
    
    canvas.save("math_examples.svg").unwrap();
    println!("Math examples saved to math_examples.svg");
}
```

## Practical Applications

### Drawing Arrows

```rust
use svg_rs::math::*;

fn draw_arrow(canvas: &mut Svg, start: Point, end: Point, head_size: f64) {
    let line = Line::new(start, end);
    
    // Draw main line
    canvas.line(start.x as i32, start.y as i32, end.x as i32, end.y as i32)
        .stroke("#333").stroke_width(2);
    
    // Calculate arrow head
    let angle = Math::angle(&start, &end, None);
    let head_angle1 = angle + Math::rad(150.0);
    let head_angle2 = angle - Math::rad(150.0);
    
    let head1_x = end.x + head_size * head_angle1.cos();
    let head1_y = end.y + head_size * head_angle1.sin();
    let head2_x = end.x + head_size * head_angle2.cos();
    let head2_y = end.y + head_size * head_angle2.sin();
    
    // Draw arrow head
    let head_path = format!("M {} {} L {} {} L {} {} Z", 
                           end.x, end.y, head1_x, head1_y, head2_x, head2_y);
    canvas.path(&head_path).fill("#333");
}
```

### Creating Regular Patterns

```rust
fn create_radial_pattern(canvas: &mut Svg, center: Point, radius: f64, count: u32) {
    let angle_step = 2.0 * std::f64::consts::PI / count as f64;
    
    for i in 0..count {
        let angle = i as f64 * angle_step;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        
        canvas.circle(5)
            .fill("#4ecdc4")
            .center(x as i32, y as i32);
    }
}
```

### Grid Snapping

```rust
fn snap_to_grid(point: Point, grid_size: f64) -> Point {
    Point::new(
        (point.x / grid_size).round() * grid_size,
        (point.y / grid_size).round() * grid_size
    )
}
```

## Performance Considerations

- Point and Line operations are lightweight
- Distance calculations use squared distance when possible to avoid sqrt()
- Angle calculations use atan2 for proper quadrant handling
- Line intersections handle parallel lines gracefully

## Mathematical Background

### Coordinate System
- SVG uses a coordinate system with origin (0,0) at top-left
- Positive X goes right, positive Y goes down
- Angles are measured clockwise from positive X-axis

### Angle Calculations
- Uses `atan2(dy, dx)` for proper quadrant handling
- Results are normalized to [0, 2π] range
- Three-point angles calculate the absolute difference

### Line Equations
- Lines are represented in implicit form: ax + by + c = 0
- Intersection uses determinant calculation
- Parallel detection uses cross product of direction vectors

## Best Practices

1. **Use appropriate precision**: f64 provides good precision for most SVG work
2. **Cache calculations**: Store frequently used values like midpoints
3. **Handle edge cases**: Check for parallel lines, zero-length segments
4. **Normalize angles**: Use consistent angle ranges in your application
5. **Combine with SVG elements**: Use math results to position and transform SVG elements
