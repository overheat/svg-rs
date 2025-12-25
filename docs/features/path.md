# Path Feature

The path feature provides advanced SVG path building capabilities with all standard path commands.

## Enabling the Feature

```toml
[dependencies]
svg-rs = { version = "0.2", features = ["path"] }
```

```rust
use svg_rs::*;
use svg_rs::path::PathBuilder;
```

## Path Commands

### Movement Commands

#### Move To (M/m)

```rust
let path = canvas.path("")
    .M(50.0, 50.0)    // Move to absolute position
    .m(10.0, 10.0);   // Move to relative position
```

### Line Commands

#### Line To (L/l)

```rust
path.L(100.0, 100.0)  // Line to absolute position
    .l(50.0, 0.0);    // Line to relative position
```

#### Horizontal Line (H/h)

```rust
path.H(150.0)         // Horizontal line to absolute x
    .h(25.0);         // Horizontal line relative
```

#### Vertical Line (V/v)

```rust
path.V(200.0)         // Vertical line to absolute y
    .v(-50.0);        // Vertical line relative
```

### Curve Commands

#### Cubic Bezier (C/c)

```rust
// Absolute cubic Bezier
path.C(100.0, 50.0,   // First control point
       150.0, 50.0,   // Second control point
       200.0, 100.0); // End point

// Relative cubic Bezier
path.c(0.0, -50.0,    // First control point (relative)
       50.0, -50.0,   // Second control point (relative)
       100.0, 0.0);   // End point (relative)
```

#### Smooth Cubic Bezier (S/s)

```rust
// Smooth cubic Bezier (reflects previous control point)
path.S(250.0, 150.0,  // Second control point
       300.0, 100.0); // End point

// Relative smooth cubic Bezier
path.s(50.0, 50.0,    // Second control point (relative)
       100.0, 0.0);   // End point (relative)
```

#### Quadratic Bezier (Q/q)

```rust
// Absolute quadratic Bezier
path.Q(150.0, 50.0,   // Control point
       200.0, 100.0); // End point

// Relative quadratic Bezier
path.q(50.0, -50.0,   // Control point (relative)
       100.0, 0.0);   // End point (relative)
```

#### Smooth Quadratic Bezier (T/t)

```rust
// Smooth quadratic Bezier (reflects previous control point)
path.T(300.0, 100.0); // End point

// Relative smooth quadratic Bezier
path.t(100.0, 0.0);   // End point (relative)
```

### Arc Commands

#### Elliptical Arc (A/a)

```rust
// Absolute arc
path.A(50.0, 30.0,    // rx, ry (radii)
       0.0,           // x-axis rotation
       0,             // large-arc-flag (0 or 1)
       1,             // sweep-flag (0 or 1)
       200.0, 150.0); // End point

// Relative arc
path.a(25.0, 25.0,    // rx, ry (radii)
       45.0,          // x-axis rotation
       1,             // large-arc-flag
       0,             // sweep-flag
       50.0, 50.0);   // End point (relative)
```

### Close Path

#### Close Path (Z)

```rust
path.Z();  // Close the path (connects to start point)
```

## Path Management

### Basic Path Operations

```rust
let mut path = canvas.path("")
    .M(50.0, 50.0)
    .L(150.0, 50.0)
    .L(150.0, 150.0)
    .L(50.0, 150.0)
    .Z();

// Get segment count
let count = path.get_segment_count();
println!("Path has {} segments", count);

// Get specific segment
if let Some(segment) = path.get_segment(0) {
    println!("First segment: {} with {} coordinates", 
             segment.command, segment.coords.len());
}
```

### Path Manipulation

```rust
// Remove a segment
path.remove_segment(2);

// Clear entire path
path.clear_path();

// Manually redraw path
path.redraw();

// Control auto-redraw
path.update(false);  // Disable auto-redraw
// ... make multiple changes ...
path.redraw();       // Manual redraw
path.update(true);   // Re-enable auto-redraw
```

### Animated Path Drawing

```rust
// Animate path drawing
path.draw_animated(3000, 0, "ease-in-out");
// duration (ms), delay (ms), easing
```

## Complete Examples

### Basic Shapes with Paths

```rust
use svg_rs::*;
use svg_rs::path::PathBuilder;

fn main() {
    let mut canvas = Svg::new(800, 600);
    
    // Rectangle using path
    let rect_path = canvas.path("")
        .M(50.0, 50.0)
        .L(150.0, 50.0)
        .L(150.0, 150.0)
        .L(50.0, 150.0)
        .Z();
    rect_path.fill("#ff6b6b").stroke("#333").stroke_width(2);
    
    // Triangle using path
    let triangle = canvas.path("")
        .M(250.0, 150.0)
        .L(300.0, 50.0)
        .L(350.0, 150.0)
        .Z();
    triangle.fill("#4ecdc4").stroke("#333").stroke_width(2);
    
    // Heart shape using curves
    let heart = canvas.path("")
        .M(500.0, 100.0)
        .C(500.0, 80.0, 480.0, 60.0, 450.0, 60.0)
        .C(420.0, 60.0, 400.0, 80.0, 400.0, 100.0)
        .C(400.0, 120.0, 450.0, 170.0, 500.0, 200.0)
        .C(550.0, 170.0, 600.0, 120.0, 600.0, 100.0)
        .C(600.0, 80.0, 580.0, 60.0, 550.0, 60.0)
        .C(520.0, 60.0, 500.0, 80.0, 500.0, 100.0)
        .Z();
    heart.fill("#e74c3c");
    
    canvas.save("path_shapes.svg").unwrap();
}
```

### Complex Curves

```rust
fn create_wave_path(canvas: &mut Svg) {
    let wave = canvas.path("")
        .M(50.0, 300.0)
        .Q(100.0, 250.0, 150.0, 300.0)  // First wave
        .T(250.0, 300.0)                // Smooth continuation
        .T(350.0, 300.0)                // Another wave
        .T(450.0, 300.0);               // Final wave
    
    wave.fill("none")
        .stroke("#3498db")
        .stroke_width(3);
}

fn create_spiral_path(canvas: &mut Svg) {
    let mut spiral = canvas.path("").M(400.0, 400.0);
    
    let center_x = 400.0;
    let center_y = 400.0;
    let mut radius = 5.0;
    let mut angle = 0.0;
    
    for _ in 0..50 {
        angle += 0.3;
        radius += 1.0;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        spiral.L(x, y);
    }
    
    spiral.fill("none").stroke("#9b59b6").stroke_width(2);
}
```

### Interactive Path Building

```rust
fn build_path_interactively() {
    let mut canvas = Svg::new(600, 400);
    
    // Start with empty path
    let mut path = canvas.path("").M(100.0, 200.0);
    
    // Add segments based on conditions
    path.L(200.0, 100.0);  // Line up
    
    // Add curve if needed
    path.Q(300.0, 50.0, 400.0, 100.0);  // Quadratic curve
    
    // Continue with smooth curve
    path.T(500.0, 200.0);  // Smooth quadratic
    
    // Add arc
    path.A(50.0, 50.0, 0.0, 0, 1, 400.0, 300.0);
    
    // Close if desired
    path.Z();
    
    path.fill("rgba(52, 152, 219, 0.3)")
        .stroke("#3498db")
        .stroke_width(2);
}
```

### Path Animation Sequences

```rust
fn create_animated_paths(canvas: &mut Svg) {
    // Drawing animation
    let drawing_path = canvas.path("")
        .M(50.0, 50.0)
        .C(50.0, 25.0, 100.0, 25.0, 100.0, 50.0)
        .S(150.0, 75.0, 150.0, 50.0)
        .L(200.0, 100.0)
        .Z();
    
    drawing_path.fill("none")
        .stroke("#e74c3c")
        .stroke_width(3)
        .draw_animated(4000, 0, "ease-in-out");
    
    // Morphing path (would require additional animation setup)
    let morph_path = canvas.path("")
        .M(300.0, 50.0)
        .L(350.0, 100.0)
        .L(300.0, 150.0)
        .L(250.0, 100.0)
        .Z();
    
    morph_path.fill("#2ecc71")
        .animate_attr("d", 
                     "M 300 50 L 350 100 L 300 150 L 250 100 Z",
                     "M 325 75 L 325 125 L 275 125 L 275 75 Z", 
                     3);
}
```

## Advanced Techniques

### Path Optimization

```rust
// Disable auto-redraw for batch operations
path.update(false);

// Add multiple segments
path.L(100.0, 100.0)
    .L(200.0, 100.0)
    .L(200.0, 200.0)
    .L(100.0, 200.0);

// Single redraw at the end
path.redraw();
path.update(true);
```

### Path Validation

```rust
fn validate_path(path: &Element) -> bool {
    let count = path.get_segment_count();
    
    if count == 0 {
        return false;
    }
    
    // Check if first segment is a move command
    if let Some(first_segment) = path.get_segment(0) {
        return first_segment.command == "M" || first_segment.command == "m";
    }
    
    false
}
```

### Path Utilities

```rust
fn create_rounded_rectangle(canvas: &mut Svg, x: f64, y: f64, 
                           width: f64, height: f64, radius: f64) -> &mut Element {
    canvas.path("")
        .M(x + radius, y)
        .H(x + width - radius)
        .A(radius, radius, 0.0, 0, 1, x + width, y + radius)
        .V(y + height - radius)
        .A(radius, radius, 0.0, 0, 1, x + width - radius, y + height)
        .H(x + radius)
        .A(radius, radius, 0.0, 0, 1, x, y + height - radius)
        .V(y + radius)
        .A(radius, radius, 0.0, 0, 1, x + radius, y)
        .Z()
}

fn create_star_path(canvas: &mut Svg, cx: f64, cy: f64, 
                   spikes: u32, inner_r: f64, outer_r: f64) -> &mut Element {
    let mut path = canvas.path("");
    let angle_step = std::f64::consts::PI / spikes as f64;
    
    for i in 0..(spikes * 2) {
        let angle = i as f64 * angle_step;
        let radius = if i % 2 == 0 { outer_r } else { inner_r };
        let x = cx + radius * angle.cos();
        let y = cy + radius * angle.sin();
        
        if i == 0 {
            path.M(x, y);
        } else {
            path.L(x, y);
        }
    }
    
    path.Z()
}
```

## Best Practices

### Path Construction

1. **Always start with M/m**: Every path should begin with a move command
2. **Use relative commands**: When appropriate, relative commands can be more intuitive
3. **Minimize path complexity**: Simpler paths render faster and are easier to maintain
4. **Close paths properly**: Use Z command for closed shapes

### Performance

1. **Batch operations**: Disable auto-redraw for multiple changes
2. **Reuse paths**: Store and reuse complex path definitions
3. **Optimize coordinates**: Use reasonable precision (avoid excessive decimal places)
4. **Group related paths**: Use SVG groups for related path elements

### Debugging

1. **Visualize control points**: Add temporary circles to show Bezier control points
2. **Step-by-step building**: Build complex paths incrementally
3. **Validate segments**: Check segment count and types during development
4. **Use path visualization tools**: External SVG viewers can help debug complex paths

## SVG Path Specification Compliance

svg-rs path feature implements the complete SVG Path specification:

- ✅ All movement commands (M, m)
- ✅ All line commands (L, l, H, h, V, v)
- ✅ All curve commands (C, c, S, s, Q, q, T, t)
- ✅ All arc commands (A, a)
- ✅ Close path command (Z)
- ✅ Path manipulation utilities
- ✅ Animation support

The implementation follows SVG 1.1 and SVG 2.0 specifications for maximum compatibility.
