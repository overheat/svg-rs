# API Reference

Complete API reference for svg-rs library.

## Core Types

### Svg

Main canvas for creating SVG documents.

```rust
impl Svg {
    pub fn new(width: u32, height: u32) -> Self
    pub fn viewbox(&mut self, x: f32, y: f32, width: f32, height: f32) -> &mut Self
    pub fn save(&self, filename: &str) -> std::io::Result<()>
    pub fn to_string(&self) -> String
}
```

### Element

Represents an SVG element with styling and transformation capabilities.

```rust
impl Element {
    // Styling
    pub fn fill(&mut self, color: &str) -> &mut Self
    pub fn stroke(&mut self, color: &str) -> &mut Self
    pub fn stroke_width(&mut self, width: u32) -> &mut Self
    pub fn opacity(&mut self, value: f64) -> &mut Self
    pub fn class(&mut self, name: &str) -> &mut Self
    pub fn style(&mut self, css: &str) -> &mut Self
    pub fn id(&mut self, id: &str) -> &mut Self
    
    // Positioning
    pub fn move_to(&mut self, x: i32, y: i32) -> &mut Self
    pub fn center(&mut self, x: i32, y: i32) -> &mut Self
    
    // Transformations
    pub fn rotate(&mut self, angle: f64) -> &mut Self
    pub fn scale(&mut self, x: f64, y: f64) -> &mut Self
    pub fn translate(&mut self, x: i32, y: i32) -> &mut Self
    pub fn skew_x(&mut self, angle: f64) -> &mut Self
    pub fn skew_y(&mut self, angle: f64) -> &mut Self
    pub fn flip(&mut self, axis: &str) -> &mut Self
    pub fn transform(&mut self, transform: &str) -> &mut Self
    
    // Animation
    pub fn animate_attr(&mut self, attr: &str, from: &str, to: &str, duration: u32) -> &mut Self
}
```

## Shape Creation Methods

### Basic Shapes

```rust
impl Svg {
    pub fn rect(&mut self, width: u32, height: u32) -> &mut Element
    pub fn circle(&mut self, radius: u32) -> &mut Element
    pub fn ellipse(&mut self, rx: u32, ry: u32) -> &mut Element
    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> &mut Element
    pub fn path(&mut self, d: &str) -> &mut Element
    pub fn polygon(&mut self, points: &str) -> &mut Element
    pub fn polyline(&mut self, points: &str) -> &mut Element
}
```

### Text Elements

```rust
impl Svg {
    pub fn text(&mut self, content: &str) -> &mut Element
}

impl Element {
    pub fn tspan(&mut self, content: &str) -> &mut Element
    pub fn text_path(&mut self, path_id: &str, content: &str) -> &mut Element
    
    // Text styling
    pub fn font_size(&mut self, size: u32) -> &mut Self
    pub fn font_family(&mut self, family: &str) -> &mut Self
    pub fn font_weight(&mut self, weight: &str) -> &mut Self
    pub fn font_style(&mut self, style: &str) -> &mut Self
    pub fn text_decoration(&mut self, decoration: &str) -> &mut Self
}
```

### Container Elements

```rust
impl Svg {
    pub fn group(&mut self) -> &mut Element
    pub fn defs(&mut self) -> &mut Element
    pub fn mask(&mut self, id: &str) -> &mut Element
    pub fn clip_path(&mut self, id: &str) -> &mut Element
    pub fn marker(&mut self, id: &str, width: u32, height: u32) -> &mut Element
    pub fn pattern(&mut self, id: &str, width: u32, height: u32) -> &mut Element
}
```

### Gradient Elements

```rust
impl Element {
    pub fn linear_gradient(&mut self, id: &str) -> &mut Element
    pub fn radial_gradient(&mut self, id: &str) -> &mut Element
    pub fn stop(&mut self, offset: &str, color: &str) -> &mut Element
}
```

## Feature-Specific APIs

### Shapes Feature

```rust
#[cfg(feature = "shapes")]
impl Svg {
    pub fn star(&mut self, spikes: u32, inner: f64, outer: f64) -> &mut Element
    pub fn ngon(&mut self, edges: u32, radius: f64) -> &mut Element
    pub fn cross(&mut self, width: f64, height: f64, thickness: f64) -> &mut Element
}
```

### Math Feature

```rust
#[cfg(feature = "math")]
pub struct Math;

impl Math {
    pub fn angle(p1: &Point, p2: &Point, p3: Option<&Point>) -> f64
    pub fn rad(degrees: f64) -> f64
    pub fn deg(radians: f64) -> f64
    pub fn lerp(a: f64, b: f64, x: f64) -> f64
    pub fn snap_to_angle(angle: f64, directions: &[f64]) -> f64
}

#[cfg(feature = "math")]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self
    pub fn distance_to(&self, other: &Point) -> f64
    pub fn angle_to(&self, other: &Point) -> f64
}

#[cfg(feature = "math")]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self
    pub fn midpoint(&self) -> Point
    pub fn interpolated_point(&self, t: f64) -> Point
    pub fn perpendicular_line(&self, p: &Point, distance: f64) -> Line
    pub fn intersection(&self, other: &Line) -> LineIntersection
    pub fn parallel(&self, other: &Line) -> bool
    pub fn closest_point(&self, p: &Point) -> Point
}
```

### Path Feature

```rust
#[cfg(feature = "path")]
pub trait PathBuilder {
    // Movement commands
    fn M(&mut self, x: f64, y: f64) -> &mut Self;
    fn m(&mut self, dx: f64, dy: f64) -> &mut Self;
    
    // Line commands
    fn L(&mut self, x: f64, y: f64) -> &mut Self;
    fn l(&mut self, dx: f64, dy: f64) -> &mut Self;
    fn H(&mut self, x: f64) -> &mut Self;
    fn h(&mut self, dx: f64) -> &mut Self;
    fn V(&mut self, y: f64) -> &mut Self;
    fn v(&mut self, dy: f64) -> &mut Self;
    
    // Curve commands
    fn C(&mut self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, x: f64, y: f64) -> &mut Self;
    fn c(&mut self, dc1x: f64, dc1y: f64, dc2x: f64, dc2y: f64, dx: f64, dy: f64) -> &mut Self;
    fn S(&mut self, c2x: f64, c2y: f64, x: f64, y: f64) -> &mut Self;
    fn s(&mut self, dc2x: f64, dc2y: f64, dx: f64, dy: f64) -> &mut Self;
    fn Q(&mut self, cx: f64, cy: f64, x: f64, y: f64) -> &mut Self;
    fn q(&mut self, dcx: f64, dcy: f64, dx: f64, dy: f64) -> &mut Self;
    fn T(&mut self, x: f64, y: f64) -> &mut Self;
    fn t(&mut self, dx: f64, dy: f64) -> &mut Self;
    
    // Arc commands
    fn A(&mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc_flag: i32, sweep_flag: i32, x: f64, y: f64) -> &mut Self;
    fn a(&mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc_flag: i32, sweep_flag: i32, dx: f64, dy: f64) -> &mut Self;
    
    // Close path
    fn Z(&mut self) -> &mut Self;
    
    // Path management
    fn clear_path(&mut self) -> &mut Self;
    fn get_segment_count(&self) -> usize;
    fn remove_segment(&mut self, index: usize) -> &mut Self;
    fn redraw(&mut self) -> &mut Self;
    fn draw_animated(&mut self, duration: u32, delay: u32, easing: &str) -> &mut Self;
}
```

### Draggable Feature

```rust
#[cfg(feature = "draggable")]
impl Element {
    pub fn draggable(&mut self) -> &mut Self
    pub fn drag_constrain(&mut self, x: f64, y: f64, width: f64, height: f64) -> &mut Self
    pub fn drag_snap_grid(&mut self, grid_size: f64) -> &mut Self
}
```

## Styling Properties

### Fill and Stroke

```rust
element
    .fill("#ff0000")                    // Solid color
    .fill("url(#gradient)")             // Gradient reference
    .fill("none")                       // No fill
    .stroke("#000000")                  // Stroke color
    .stroke_width(2)                    // Stroke width
    .stroke_dasharray("5,5")            // Dashed stroke
    .stroke_linecap("round")            // Line cap: butt, round, square
    .stroke_linejoin("round")           // Line join: miter, round, bevel
    .stroke_opacity(0.5)                // Stroke opacity
    .fill_opacity(0.8);                 // Fill opacity
```

### Text Properties

```rust
text_element
    .font_size(16)                      // Font size in pixels
    .font_family("Arial, sans-serif")   // Font family
    .font_weight("bold")                // Font weight: normal, bold, 100-900
    .font_style("italic")               // Font style: normal, italic, oblique
    .text_decoration("underline")       // Text decoration
    .text_anchor("middle")              // Text anchor: start, middle, end
    .dominant_baseline("central");      // Baseline alignment
```

### Transform Properties

```rust
element
    .rotate(45.0)                       // Rotate in degrees
    .scale(2.0, 1.5)                    // Scale x, y
    .translate(10, 20)                  // Translate x, y
    .skew_x(15.0)                       // Skew X in degrees
    .skew_y(10.0)                       // Skew Y in degrees
    .flip("x")                          // Flip: "x", "y", or "xy"
    .transform("matrix(1,0,0,1,0,0)");  // Custom transform matrix
```

## Animation Properties

### Basic Animation

```rust
element.animate_attr(
    "fill",                             // Attribute to animate
    "#ff0000",                          // From value
    "#0000ff",                          // To value
    3                                   // Duration in seconds
);
```

### Transform Animation

```rust
element.animate_attr(
    "transform",
    "rotate(0)",
    "rotate(360)",
    2
);
```

## Constants and Enums

### Color Constants

```rust
// Common colors (as string literals)
"red", "green", "blue", "black", "white"
"transparent", "none"

// Hex colors
"#ff0000", "#00ff00", "#0000ff"

// RGB/RGBA
"rgb(255, 0, 0)", "rgba(255, 0, 0, 0.5)"

// HSL/HSLA
"hsl(0, 100%, 50%)", "hsla(0, 100%, 50%, 0.5)"
```

### Transform Functions

```rust
// Available transform functions (as strings)
"translate(x, y)"
"rotate(angle)"
"scale(x, y)"
"skewX(angle)"
"skewY(angle)"
"matrix(a, b, c, d, e, f)"
```

### Animation Easing

```rust
// Easing functions (as strings)
"linear"
"ease"
"ease-in"
"ease-out"
"ease-in-out"
"cubic-bezier(0.1, 0.7, 1.0, 0.1)"
```

## Error Handling

### Result Types

```rust
// File operations return Result
canvas.save("output.svg"): Result<(), std::io::Error>

// Most other operations return &mut Self for chaining
element.fill("#red"): &mut Element
```

### Common Error Scenarios

```rust
// File write errors
match canvas.save("output.svg") {
    Ok(()) => println!("Saved successfully"),
    Err(e) => eprintln!("Error saving file: {}", e),
}

// Invalid attribute values are generally accepted but may not render correctly
element.fill("invalid-color");  // Won't panic, but may not display as expected
```

## Best Practices

### Method Chaining

```rust
// Preferred: chain methods for cleaner code
canvas.rect(100, 100)
    .fill("#ff6b6b")
    .stroke("#333")
    .stroke_width(2)
    .move_to(50, 50)
    .rotate(45.0);

// Avoid: separate method calls
let rect = canvas.rect(100, 100);
rect.fill("#ff6b6b");
rect.stroke("#333");
// ... etc
```

### Resource Management

```rust
// Use defs for reusable elements
let defs = canvas.defs();
let gradient = defs.linear_gradient("myGradient");
gradient.stop("0%", "#ff0000");
gradient.stop("100%", "#0000ff");

// Reference in multiple elements
canvas.rect(100, 100).fill("url(#myGradient)");
canvas.circle(50).fill("url(#myGradient)");
```

### Performance Considerations

```rust
// Group related elements
let group = canvas.group().transform("translate(100, 100)");
group.circle(10).fill("#red");
group.rect(20, 20).fill("#blue");

// Use appropriate precision for coordinates
element.move_to(100, 200);  // Good
element.move_to(100.123456789, 200.987654321);  // Excessive precision
```

## Version Compatibility

### Current Version: 0.2.0

- ✅ All core SVG functionality
- ✅ Optional features: shapes, math, path, draggable
- ✅ Animation support
- ✅ Gradient and pattern support
- ✅ Text processing with tspan
- ✅ Transform operations
- ✅ CSS integration

### Breaking Changes

None in current version. Future versions will follow semantic versioning.
