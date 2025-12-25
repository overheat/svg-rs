# Core SVG Functionality

This document covers the core SVG functionality available without any optional features.

## Canvas Management

### Creating a Canvas

```rust
use svg_rs::*;

let mut canvas = Svg::new(800, 600);
```

### Viewbox

Set a custom viewbox for scaling:

```rust
canvas.viewbox(0.0, 0.0, 100.0, 100.0);
```

### Saving

```rust
// Save to file
canvas.save("output.svg")?;

// Get as string
let svg_string = canvas.to_string();
```

## Basic Shapes

### Rectangle

```rust
canvas.rect(width, height)
    .fill("#ff6b6b")
    .stroke("#333")
    .stroke_width(2)
    .move_to(x, y);
```

### Circle

```rust
canvas.circle(radius)
    .fill("#4ecdc4")
    .center(cx, cy);
```

### Ellipse

```rust
canvas.ellipse(rx, ry)
    .fill("#9b59b6")
    .center(cx, cy);
```

### Line

```rust
canvas.line(x1, y1, x2, y2)
    .stroke("#333")
    .stroke_width(2);
```

### Path

```rust
canvas.path("M 10 10 L 90 90 Z")
    .fill("none")
    .stroke("#333");
```

### Polygon

```rust
canvas.polygon("0,0 50,0 25,50")
    .fill("#e74c3c");
```

### Polyline

```rust
canvas.polyline("0,0 50,25 100,0")
    .fill("none")
    .stroke("#333");
```

## Text

### Basic Text

```rust
canvas.text("Hello World")
    .move_to(50, 100)
    .font_size(24)
    .font_family("Arial")
    .fill("#333");
```

### Text Styling

```rust
canvas.text("Styled Text")
    .font_size(18)
    .font_family("serif")
    .font_weight("bold")
    .font_style("italic")
    .text_decoration("underline")
    .fill("#ff6b6b");
```

### Advanced Text with Tspan

```rust
let text = canvas.text("")
    .move_to(50, 100);

text.tspan("Hello ").fill("#000");
text.tspan("World!").fill("#f06").font_weight("bold");
```

## Images

```rust
canvas.image("path/to/image.jpg", 200, 150)
    .move_to(100, 100);
```

## Groups

```rust
let group = canvas.group()
    .transform("translate(100, 100)");

group.circle(30).fill("#red");
group.rect(40, 40).fill("#blue").move_to(-20, 40);
```

## Styling Methods

### Fill and Stroke

```rust
element
    .fill("#ff0000")           // Solid color
    .fill("url(#gradient)")    // Gradient
    .fill("none")              // No fill
    .stroke("#000000")         // Stroke color
    .stroke_width(2)           // Stroke width
    .stroke_dasharray("5,5")   // Dashed stroke
    .stroke_linecap("round");  // Line cap style
```

### Opacity

```rust
element
    .opacity(0.5)              // Overall opacity
    .fill_opacity(0.8)         // Fill opacity
    .stroke_opacity(0.6);      // Stroke opacity
```

### CSS Classes and Styles

```rust
element
    .class("my-class")         // Add CSS class
    .style("fill: red; stroke: blue;"); // Inline styles
```

## Positioning and Transforms

### Basic Positioning

```rust
element
    .move_to(x, y)             // Move to position
    .center(cx, cy);           // Center (circles/ellipses)
```

### Transformations

```rust
element
    .rotate(45.0)              // Rotate in degrees
    .scale(2.0, 1.5)           // Scale x, y
    .translate(10, 20)         // Translate
    .skew_x(15.0)              // Skew X
    .skew_y(10.0)              // Skew Y
    .flip("x")                 // Flip horizontally
    .flip("y");                // Flip vertically
```

### Transform Matrix

```rust
element.transform("matrix(1,0,0,1,30,40)");
```

## Gradients and Patterns

### Linear Gradient

```rust
let defs = canvas.defs();
let gradient = defs.linear_gradient("myGradient");
gradient.stop("0%", "#ff0000");
gradient.stop("100%", "#0000ff");

// Use gradient
canvas.rect(200, 100)
    .fill("url(#myGradient)");
```

### Radial Gradient

```rust
let gradient = defs.radial_gradient("radialGrad");
gradient.stop("0%", "#ffffff");
gradient.stop("100%", "#000000");
```

### Pattern

```rust
let pattern = defs.pattern("myPattern", 20, 20);
pattern.rect(10, 10).fill("#ff0000");
pattern.circle(5).fill("#0000ff").center(15, 15);

canvas.rect(100, 100).fill("url(#myPattern)");
```

## Masks and Clipping

### Mask

```rust
let mask = canvas.mask("myMask");
mask.rect(100, 100).fill("white");
mask.circle(50).fill("black").center(50, 50);

canvas.rect(100, 100)
    .fill("#ff0000")
    .mask("url(#myMask)");
```

### Clipping Path

```rust
let clip = canvas.clip_path("myClip");
clip.circle(50).center(50, 50);

canvas.rect(100, 100)
    .fill("#ff0000")
    .clip_path("url(#myClip)");
```

## Animations

### Basic Animation

```rust
canvas.circle(25)
    .fill("#0f6")
    .center(100, 100)
    .animate_attr("r", "25", "50", 2); // attribute, from, to, duration
```

### Transform Animation

```rust
canvas.rect(50, 50)
    .fill("#f06")
    .animate_attr("transform", "rotate(0)", "rotate(360)", 3);
```

## Markers

```rust
let marker = canvas.marker("arrow", 10, 10);
marker.path("M 0 0 L 10 5 L 0 10 Z").fill("#333");

canvas.line(50, 50, 200, 100)
    .stroke("#333")
    .marker_end("url(#arrow)");
```

## Best Practices

1. **Method Chaining**: Use fluent API for cleaner code
2. **Grouping**: Group related elements for better organization
3. **Reusable Definitions**: Use defs for gradients, patterns, and markers
4. **Semantic Naming**: Use meaningful IDs and classes
5. **Performance**: Minimize DOM manipulation by batching operations

## Error Handling

```rust
use std::io::Result;

fn create_svg() -> Result<()> {
    let mut canvas = Svg::new(400, 300);
    canvas.rect(100, 100).fill("#red");
    canvas.save("output.svg")?;
    Ok(())
}
```
