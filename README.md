# SVG-RS Documentation

Welcome to svg-rs, a lightweight Rust library for creating and manipulating SVG graphics, inspired by svg.js.

## Table of Contents

- [Getting Started](getting-started.md)
- [Core SVG Functionality](core-svg.md)
- [Features](features/)
  - [Shapes Feature](features/shapes.md)
  - [Math Feature](features/math.md)
  - [Path Feature](features/path.md)
  - [Draggable Feature](features/draggable.md)
- [Examples](examples.md)
- [API Reference](api-reference.md)

## Quick Start

```rust
use svg_rs::*;

let mut canvas = Svg::new(800, 600);

// Create a rectangle
canvas.rect(100, 100)
    .fill("#f06")
    .move_to(50, 50);

// Create a circle
canvas.circle(50)
    .fill("#0f6")
    .center(200, 100);

// Save to file
canvas.save("output.svg").unwrap();
```

## Features

svg-rs provides several optional features that can be enabled as needed:

- **shapes** - Advanced geometric shapes (stars, polygons, crosses)
- **math** - Mathematical utilities for geometric calculations
- **path** - Advanced SVG path building with all commands
- **draggable** - Interactive drag functionality

Enable features in your `Cargo.toml`:

```toml
[dependencies]
svg-rs = { version = "0.2", features = ["shapes", "math", "path"] }
```

## License

This project is licensed under the MIT License.
