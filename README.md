# svg-rs

[![Crates.io](https://img.shields.io/crates/v/svg-rs.svg)](https://crates.io/crates/svg-rs)
[![Documentation](https://docs.rs/svg-rs/badge.svg)](https://docs.rs/svg-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/your-username/svg-rs/workflows/CI/badge.svg)](https://github.com/your-username/svg-rs/actions)

A lightweight Rust library for creating and manipulating SVG graphics, inspired by [svg.js](https://github.com/svgdotjs/svg.js).

**[中文文档](README_zh.md) | [Examples](examples/) | [API Documentation](https://docs.rs/svg-rs)**

## ✨ Features

- 🎨 **Complete SVG element support**: rectangles, circles, ellipses, lines, paths, text, images, and more
- 🔗 **Fluent API design**: chainable methods similar to svg.js
- 🎭 **Advanced graphics**: gradients, patterns, masks, clipping paths
- 🎬 **Animations**: SVG animations with attribute interpolation
- 📝 **Text processing**: advanced text handling with tspan and textPath
- 🔄 **Transformations**: rotate, scale, translate, skew, flip
- 🎨 **CSS integration**: class management and inline styles
- 🖱️ **Draggable elements**: interactive drag functionality (optional feature)
- 🧮 **Mathematical utilities**: geometric calculations, angles, lines, points (optional feature)
- 🚀 **Zero dependencies**: pure Rust implementation
- 🛡️ **Type safety**: leverages Rust's type system for correctness
- 📦 **Lightweight**: minimal footprint with maximum functionality

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
svg-rs = "0.1"

# Enable draggable feature for interactive elements
svg-rs = { version = "0.1", features = ["draggable"] }

# Enable shapes feature for advanced geometric shapes
svg-rs = { version = "0.1", features = ["shapes"] }

# Enable math feature for mathematical utilities
svg-rs = { version = "0.1", features = ["math"] }

# Enable multiple features
svg-rs = { version = "0.1", features = ["draggable", "shapes", "math"] }
```

### Basic Usage

```rust
use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 600);

    // Create a rectangle
    canvas.rect(100, 100)
        .fill("#f06")
        .move_to(50, 50);

    // Create a circle with animation
    let circle = canvas.circle(25)
        .fill("#0f6")
        .center(200, 100);
    circle.animate_attr("r", "25", "50", 2);

    // Save to file
    canvas.save("output.svg").unwrap();
    println!("SVG saved to output.svg");
}
```

## 📚 Examples

### Advanced Shapes

```rust
// Enable shapes feature for stars and regular polygons
let mut canvas = Svg::new(400, 300);

// Create a 5-pointed star
canvas.star(5, 30.0, 60.0)
    .fill("#ff6b6b")
    .stroke("#333")
    .stroke_width(2)
    .move_to(100, 100);

// Create a hexagon (6-sided regular polygon)
canvas.ngon(6, 40.0)
    .fill("#4ecdc4")
    .stroke("#333")
    .stroke_width(2)
    .move_to(250, 100);
```

### Gradients and Patterns

```rust
let mut canvas = Svg::new(400, 300);

// Create gradient
let defs = canvas.defs();
let gradient = defs.linear_gradient("myGradient");
gradient.stop("0%", "#ff0000");
gradient.stop("100%", "#0000ff");

// Use gradient
canvas.rect(200, 100)
    .fill("url(#myGradient)")
    .move_to(50, 50);
```

### Advanced Text Processing

```rust
let text = canvas.text("")
    .move_to(50, 100)
    .font_family("Arial")
    .font_size(18);

text.tspan("Hello ").fill("#000");
text.tspan("World!").fill("#f06").font_weight("bold");
```

### Transformations and Animations

```rust
// Complex transformations
canvas.rect(60, 60)
    .fill("#f0f")
    .move_to(100, 100)
    .rotate(45.0)
    .skew_x(15.0)
    .animate_attr("fill", "#f0f", "#0ff", 3);

// Groups with transforms
let group = canvas.group()
    .transform("translate(100, 100) scale(1.5)");

group.circle(30).fill("#red").center(0, 0);
group.rect(40, 40).fill("#blue").move_to(-20, 40);
```

### Mathematical Utilities

```rust
// Enable math feature for geometric calculations
use svg_rs::math::*;

let p1 = Point::new(0.0, 0.0);
let p2 = Point::new(100.0, 100.0);

// Calculate angle between points
let angle = Math::angle(&p1, &p2, None);
println!("Angle: {} degrees", Math::deg(angle));

// Create a line and find its midpoint
let line = Line::new(p1, p2);
let midpoint = line.midpoint();

// Create perpendicular line
let perp = line.perpendicular_line(&midpoint, 50.0);

// Linear interpolation
let halfway_point = line.interpolated_point(0.5);

// Distance calculation
let distance = p1.distance_to(&p2);
```

```rust
// Enable dragging (requires "draggable" feature)
canvas.rect(120, 40)
    .fill("#2ecc71")
    .move_to(50, 200)
    .id("my-rect")
    .draggable();

// Draggable with constraints
canvas.circle(30)
    .fill("#e74c3c")
    .center(200, 200)
    .id("constrained-circle")
    .draggable()
    .drag_constrain(100.0, 100.0, 300.0, 200.0);

// Draggable with grid snapping
canvas.rect(60, 60)
    .fill("#3498db")
    .move_to(300, 100)
    .id("grid-rect")
    .draggable()
    .drag_snap_grid(25.0);
```

## 📖 API Reference

### Creating Elements
| Method | Description | Example |
|--------|-------------|---------|
| `rect(width, height)` | Rectangle | `canvas.rect(100, 50)` |
| `circle(radius)` | Circle | `canvas.circle(25)` |
| `ellipse(rx, ry)` | Ellipse | `canvas.ellipse(50, 30)` |
| `line(x1, y1, x2, y2)` | Line | `canvas.line(0, 0, 100, 100)` |
| `path(d)` | Path | `canvas.path("M 10 10 L 90 90")` |
| `text(content)` | Text | `canvas.text("Hello")` |
| `image(href, w, h)` | Image | `canvas.image("pic.jpg", 100, 100)` |
| `polygon(points)` | Polygon | `canvas.polygon("0,0 50,0 25,50")` |
| `polyline(points)` | Polyline | `canvas.polyline("0,0 50,25 100,0")` |
| `star(spikes, inner, outer)` | Star shape | `canvas.star(5, 30.0, 60.0)` |
| `ngon(edges, radius)` | Regular polygon | `canvas.ngon(6, 40.0)` |
| `cross(width, height, thickness)` | Cross shape | `canvas.cross(60.0, 80.0, 20.0)` |

### Math Utilities (with `math` feature)
| Method | Description | Example |
|--------|-------------|---------|
| `Math::angle(p1, p2, p3?)` | Calculate angle | `Math::angle(&p1, &p2, None)` |
| `Math::rad(degrees)` | Degrees to radians | `Math::rad(45.0)` |
| `Math::deg(radians)` | Radians to degrees | `Math::deg(PI/4.0)` |
| `Math::lerp(a, b, t)` | Linear interpolation | `Math::lerp(0.0, 100.0, 0.5)` |
| `Math::snap_to_angle(angle, dirs)` | Snap to nearest angle | `Math::snap_to_angle(angle, &[0.0, PI/2.0])` |
| `Point::new(x, y)` | Create point | `Point::new(10.0, 20.0)` |
| `Point::distance_to(other)` | Distance between points | `p1.distance_to(&p2)` |
| `Line::new(p1, p2)` | Create line | `Line::new(p1, p2)` |
| `Line::midpoint()` | Line midpoint | `line.midpoint()` |
| `Line::perpendicular_line(p, dist)` | Perpendicular line | `line.perpendicular_line(&p, 50.0)` |
| `Line::interpolated_point(t)` | Point at parameter t | `line.interpolated_point(0.5)` |

### Styling Methods
| Method | Description | Example |
|--------|-------------|---------|
| `fill(color)` | Fill color | `.fill("#ff0000")` |
| `stroke(color)` | Stroke color | `.stroke("#000000")` |
| `stroke_width(width)` | Stroke width | `.stroke_width(2)` |
| `opacity(value)` | Opacity | `.opacity(0.5)` |
| `class(name)` | CSS class | `.class("highlight")` |
| `style(css)` | Inline styles | `.style("fill: red;")` |

### Positioning and Transforms
| Method | Description | Example |
|--------|-------------|---------|
| `move_to(x, y)` | Position | `.move_to(50, 100)` |
| `center(x, y)` | Center point | `.center(100, 100)` |
| `rotate(angle)` | Rotation | `.rotate(45.0)` |
| `scale(x, y)` | Scaling | `.scale(2.0, 1.5)` |
| `translate(x, y)` | Translation | `.translate(10, 20)` |
| `skew(x, y)` | Skewing | `.skew(15.0, 0.0)` |
| `flip(axis)` | Flipping | `.flip("x")` |

### Advanced Features
| Method | Description | Example |
|--------|-------------|---------|
| `animate_attr(attr, from, to, dur)` | Animation | `.animate_attr("r", "10", "50", 2)` |
| `draggable()` | Enable dragging | `.draggable()` |
| `drag_constrain(x, y, w, h)` | Drag constraints | `.drag_constrain(0, 0, 100, 100)` |
| `drag_snap_grid(size)` | Grid snapping | `.drag_snap_grid(25.0)` |
| `mask(id)` | Apply mask | `.mask("myMask")` |
| `clip_path(id)` | Apply clipping | `.clip_path("myClip")` |
| `marker_start/mid/end(id)` | Path markers | `.marker_end("arrow")` |
| `viewbox(x, y, w, h)` | Set viewbox | `canvas.viewbox(0, 0, 100, 100)` |

## 🎯 Comparison with svg.js

svg-rs provides approximately **95%** of svg.js functionality with the benefits of Rust's type safety and performance:

| Feature | svg.js | svg-rs | Status |
|---------|--------|--------|--------|
| Basic elements | ✅ | ✅ | Complete |
| Animations | ✅ | ✅ | Complete |
| Gradients/Patterns | ✅ | ✅ | Complete |
| Text processing | ✅ | ✅ | Complete |
| Transformations | ✅ | ✅ | Complete |
| CSS integration | ✅ | ✅ | Complete |
| Groups | ✅ | ✅ | Complete |
| Masks/Clipping | ✅ | ✅ | Complete |
| Events | ✅ | ✅ | Basic support |
| DOM manipulation | ✅ | ⚠️ | Partial |

## 🏃‍♂️ Running Examples

Check out the [examples](examples/) directory for comprehensive usage examples:

```bash
# Basic shapes and styling
cargo run --example shapes

# Advanced shapes (stars and polygons)
cargo run --example shapes --features shapes

# Advanced features demo  
cargo run --example advanced

# Gradients and patterns
cargo run --example gradients

# Animations
cargo run --example animations

# Mathematical utilities
cargo run --example math --features math

# Interactive draggable elements
cargo run --example draggable --features draggable

# Interactive elements
cargo run --example events --features draggable
```

## 🛠️ Development

### Prerequisites

- Rust 1.70.0 or later
- Cargo

### Building

```bash
git clone https://github.com/your-username/svg-rs
cd svg-rs
cargo build
```

### Testing

```bash
cargo test
```

### Documentation

```bash
cargo doc --open
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [svg.js](https://github.com/svgdotjs/svg.js) - The JavaScript SVG library
- Built with ❤️ in Rust
- Thanks to all contributors and the Rust community

## 🔗 Related Projects

- [svg.js](https://github.com/svgdotjs/svg.js) - The original JavaScript library
- [resvg](https://github.com/RazrFalcon/resvg) - SVG rendering library
- [usvg](https://github.com/RazrFalcon/resvg/tree/master/crates/usvg) - SVG parser

---

**Star ⭐ this repository if you find it helpful!**
