# Getting Started with svg-rs

## Installation

Add svg-rs to your `Cargo.toml`:

```toml
[dependencies]
svg-rs = "0.2"
```

Or with specific features:

```toml
[dependencies]
svg-rs = { version = "0.2", features = ["shapes", "math", "path", "draggable"] }
```

## Your First SVG

```rust
use svg_rs::*;

fn main() {
    // Create a new SVG canvas
    let mut canvas = Svg::new(400, 300);
    
    // Add a rectangle
    canvas.rect(100, 80)
        .fill("#ff6b6b")
        .stroke("#333")
        .stroke_width(2)
        .move_to(50, 50);
    
    // Add a circle
    canvas.circle(40)
        .fill("#4ecdc4")
        .center(200, 100);
    
    // Add text
    canvas.text("Hello SVG!")
        .move_to(50, 200)
        .font_size(24)
        .fill("#333");
    
    // Save to file
    canvas.save("my_first_svg.svg").unwrap();
    println!("SVG saved to my_first_svg.svg");
}
```

## Basic Concepts

### Canvas Creation

```rust
let mut canvas = Svg::new(width, height);
```

### Element Creation

All elements return a mutable reference for method chaining:

```rust
canvas.rect(100, 50)
    .fill("#red")
    .stroke("#black")
    .move_to(10, 10);
```

### Styling

Elements can be styled using various methods:

```rust
element
    .fill("#ff0000")           // Fill color
    .stroke("#000000")         // Stroke color
    .stroke_width(2)           // Stroke width
    .opacity(0.8)              // Opacity
    .class("my-class");        // CSS class
```

### Positioning

```rust
element
    .move_to(x, y)             // Position element
    .center(x, y);             // Center element (for circles/ellipses)
```

### Transformations

```rust
element
    .rotate(45.0)              // Rotate in degrees
    .scale(2.0, 1.5)           // Scale x, y
    .translate(10, 20)         // Translate
    .skew(15.0, 0.0);          // Skew
```

## Next Steps

- Learn about [Core SVG Functionality](core-svg.md)
- Explore [Features](features/) for advanced capabilities
- Check out [Examples](examples.md) for more complex use cases
