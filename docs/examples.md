# Examples

This document provides comprehensive examples for using svg-rs in various scenarios.

## Basic Examples

### Hello World

```rust
use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(400, 200);
    
    canvas.text("Hello, SVG World!")
        .move_to(50, 100)
        .font_size(24)
        .font_family("Arial")
        .fill("#333");
    
    canvas.save("hello_world.svg").unwrap();
}
```

### Basic Shapes

```rust
use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(600, 400);
    
    // Rectangle
    canvas.rect(100, 80)
        .fill("#ff6b6b")
        .stroke("#333")
        .stroke_width(2)
        .move_to(50, 50);
    
    // Circle
    canvas.circle(40)
        .fill("#4ecdc4")
        .center(250, 90);
    
    // Ellipse
    canvas.ellipse(60, 30)
        .fill("#9b59b6")
        .center(400, 90);
    
    // Line
    canvas.line(50, 200, 550, 200)
        .stroke("#333")
        .stroke_width(3);
    
    // Polygon (triangle)
    canvas.polygon("300,250 350,350 250,350")
        .fill("#e74c3c");
    
    canvas.save("basic_shapes.svg").unwrap();
}
```

## Intermediate Examples

### Gradients and Patterns

```rust
use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(600, 400);
    
    // Create definitions section
    let defs = canvas.defs();
    
    // Linear gradient
    let linear_grad = defs.linear_gradient("linearGrad");
    linear_grad.stop("0%", "#ff6b6b");
    linear_grad.stop("50%", "#4ecdc4");
    linear_grad.stop("100%", "#45b7d1");
    
    // Radial gradient
    let radial_grad = defs.radial_gradient("radialGrad");
    radial_grad.stop("0%", "#ffffff");
    radial_grad.stop("70%", "#ff6b6b");
    radial_grad.stop("100%", "#c0392b");
    
    // Pattern
    let pattern = defs.pattern("checkerboard", 20, 20);
    pattern.rect(10, 10).fill("#333");
    pattern.rect(10, 10).fill("#fff").move_to(10, 10);
    pattern.rect(10, 10).fill("#fff").move_to(0, 10);
    pattern.rect(10, 10).fill("#333").move_to(10, 0);
    
    // Use gradients and patterns
    canvas.rect(150, 100)
        .fill("url(#linearGrad)")
        .move_to(50, 50);
    
    canvas.circle(60)
        .fill("url(#radialGrad)")
        .center(350, 100);
    
    canvas.rect(120, 120)
        .fill("url(#checkerboard)")
        .move_to(450, 40);
    
    canvas.save("gradients_patterns.svg").unwrap();
}
```

### Animations

```rust
use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(600, 400);
    
    // Rotating rectangle
    let rotating_rect = canvas.rect(80, 80)
        .fill("#e74c3c")
        .move_to(100, 100);
    rotating_rect.animate_attr("transform", "rotate(0 140 140)", "rotate(360 140 140)", 3);
    
    // Pulsing circle
    let pulsing_circle = canvas.circle(30)
        .fill("#2ecc71")
        .center(300, 140);
    pulsing_circle.animate_attr("r", "30", "60", 2);
    
    // Color changing ellipse
    let color_ellipse = canvas.ellipse(50, 30)
        .fill("#3498db")
        .center(450, 140);
    color_ellipse.animate_attr("fill", "#3498db", "#e67e22", 4);
    
    // Moving line
    let moving_line = canvas.line(50, 250, 150, 250)
        .stroke("#9b59b6")
        .stroke_width(4);
    moving_line.animate_attr("x2", "150", "550", 5);
    
    canvas.save("animations.svg").unwrap();
}
```

## Advanced Examples

### Complex Compositions

```rust
use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 600);
    
    // Background
    canvas.rect(800, 600)
        .fill("linear-gradient(45deg, #667eea 0%, #764ba2 100%)");
    
    // Title
    canvas.text("Complex SVG Composition")
        .move_to(50, 50)
        .font_size(28)
        .font_family("Arial, sans-serif")
        .fill("white")
        .font_weight("bold");
    
    // Create a complex shape using groups
    let main_group = canvas.group()
        .transform("translate(400, 300)");
    
    // Central circle
    main_group.circle(80)
        .fill("rgba(255, 255, 255, 0.9)")
        .stroke("#333")
        .stroke_width(3);
    
    // Surrounding elements
    for i in 0..8 {
        let angle = i as f64 * 45.0;
        let group = main_group.group()
            .transform(&format!("rotate({}) translate(120, 0)", angle));
        
        group.rect(30, 15)
            .fill("#ff6b6b")
            .move_to(-15, -7)
            .animate_attr("fill", "#ff6b6b", "#4ecdc4", 2);
        
        group.circle(8)
            .fill("#333")
            .center(0, 0);
    }
    
    // Decorative elements
    let deco_group = canvas.group()
        .transform("translate(100, 450)");
    
    for i in 0..20 {
        let x = i * 30;
        let height = 20 + (i * 5) % 40;
        
        deco_group.rect(20, height)
            .fill(&format!("hsl({}, 70%, 60%)", i * 18))
            .move_to(x, -height);
    }
    
    canvas.save("complex_composition.svg").unwrap();
}
```

### Data Visualization

```rust
use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 500);
    
    // Sample data
    let data = vec![23, 45, 56, 78, 32, 67, 89, 43, 76, 54];
    let max_value = *data.iter().max().unwrap() as f64;
    
    // Chart dimensions
    let chart_x = 100.0;
    let chart_y = 50.0;
    let chart_width = 600.0;
    let chart_height = 300.0;
    let bar_width = chart_width / data.len() as f64 * 0.8;
    let bar_spacing = chart_width / data.len() as f64;
    
    // Title
    canvas.text("Data Visualization Example")
        .move_to(50, 30)
        .font_size(20)
        .font_family("Arial")
        .fill("#333");
    
    // Chart background
    canvas.rect(chart_width as u32, chart_height as u32)
        .fill("#f8f9fa")
        .stroke("#dee2e6")
        .stroke_width(1)
        .move_to(chart_x as i32, chart_y as i32);
    
    // Bars
    for (i, &value) in data.iter().enumerate() {
        let bar_height = (value as f64 / max_value) * chart_height;
        let x = chart_x + i as f64 * bar_spacing + (bar_spacing - bar_width) / 2.0;
        let y = chart_y + chart_height - bar_height;
        
        // Bar
        let bar = canvas.rect(bar_width as u32, bar_height as u32)
            .fill("#3498db")
            .stroke("#2980b9")
            .stroke_width(1)
            .move_to(x as i32, y as i32);
        
        // Animation
        bar.animate_attr("height", "0", &bar_height.to_string(), 2);
        bar.animate_attr("y", &(chart_y + chart_height).to_string(), &y.to_string(), 2);
        
        // Value label
        canvas.text(&value.to_string())
            .move_to(x as i32 + bar_width as i32 / 2 - 8, y as i32 - 5)
            .font_size(12)
            .fill("#333");
        
        // X-axis label
        canvas.text(&format!("Item {}", i + 1))
            .move_to(x as i32, (chart_y + chart_height + 20.0) as i32)
            .font_size(10)
            .fill("#666");
    }
    
    // Y-axis
    canvas.line(chart_x as i32, chart_y as i32, 
               chart_x as i32, (chart_y + chart_height) as i32)
        .stroke("#333")
        .stroke_width(2);
    
    // X-axis
    canvas.line(chart_x as i32, (chart_y + chart_height) as i32,
               (chart_x + chart_width) as i32, (chart_y + chart_height) as i32)
        .stroke("#333")
        .stroke_width(2);
    
    canvas.save("data_visualization.svg").unwrap();
}
```

## Feature-Specific Examples

### Using Shapes Feature

```rust
#[cfg(feature = "shapes")]
use svg_rs::*;

#[cfg(feature = "shapes")]
fn main() {
    let mut canvas = Svg::new(600, 400);
    
    // Stars
    canvas.star(5, 30.0, 60.0)
        .fill("#ff6b6b")
        .move_to(100, 100);
    
    canvas.star(8, 25.0, 50.0)
        .fill("#4ecdc4")
        .move_to(250, 100);
    
    // Regular polygons
    canvas.ngon(6, 40.0)
        .fill("#9b59b6")
        .move_to(400, 100);
    
    // Cross
    canvas.cross(60.0, 80.0, 20.0)
        .fill("#e74c3c")
        .move_to(500, 60);
    
    canvas.save("shapes_example.svg").unwrap();
}

#[cfg(not(feature = "shapes"))]
fn main() {
    println!("Enable shapes feature: cargo run --features shapes");
}
```

### Using Math Feature

```rust
#[cfg(feature = "math")]
use svg_rs::*;
#[cfg(feature = "math")]
use svg_rs::math::*;

#[cfg(feature = "math")]
fn main() {
    let mut canvas = Svg::new(600, 400);
    
    // Create points
    let center = Point::new(300.0, 200.0);
    let radius = 100.0;
    
    // Draw circle of points using math
    for i in 0..12 {
        let angle = i as f64 * 2.0 * std::f64::consts::PI / 12.0;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        
        canvas.circle(8)
            .fill("#3498db")
            .center(x as i32, y as i32);
        
        // Draw line to center
        canvas.line(center.x as i32, center.y as i32, x as i32, y as i32)
            .stroke("#ecf0f1")
            .stroke_width(1);
    }
    
    // Calculate and display distances
    let p1 = Point::new(100.0, 100.0);
    let p2 = Point::new(500.0, 300.0);
    let distance = p1.distance_to(&p2);
    
    canvas.text(&format!("Distance: {:.1}", distance))
        .move_to(50, 50)
        .font_size(14)
        .fill("#333");
    
    canvas.save("math_example.svg").unwrap();
}

#[cfg(not(feature = "math"))]
fn main() {
    println!("Enable math feature: cargo run --features math");
}
```

### Using Path Feature

```rust
#[cfg(feature = "path")]
use svg_rs::*;
#[cfg(feature = "path")]
use svg_rs::path::PathBuilder;

#[cfg(feature = "path")]
fn main() {
    let mut canvas = Svg::new(600, 400);
    
    // Complex path with curves
    let path = canvas.path("")
        .M(50.0, 200.0)
        .C(50.0, 100.0, 150.0, 100.0, 150.0, 200.0)
        .S(250.0, 300.0, 250.0, 200.0)
        .Q(300.0, 150.0, 350.0, 200.0)
        .T(450.0, 200.0)
        .A(50.0, 30.0, 0.0, 0, 1, 550.0, 200.0);
    
    path.fill("none")
        .stroke("#e74c3c")
        .stroke_width(3);
    
    // Animated path drawing
    path.draw_animated(4000, 0, "ease-in-out");
    
    canvas.save("path_example.svg").unwrap();
}

#[cfg(not(feature = "path"))]
fn main() {
    println!("Enable path feature: cargo run --features path");
}
```

### Using Draggable Feature

```rust
#[cfg(feature = "draggable")]
use svg_rs::*;

#[cfg(feature = "draggable")]
fn main() {
    let mut canvas = Svg::new(600, 400);
    
    // Draggable elements
    canvas.rect(80, 80)
        .fill("#ff6b6b")
        .move_to(100, 100)
        .id("drag-rect")
        .draggable();
    
    canvas.circle(40)
        .fill("#4ecdc4")
        .center(300, 140)
        .id("drag-circle")
        .draggable()
        .drag_constrain(200.0, 100.0, 200.0, 100.0);
    
    canvas.save("draggable_example.svg").unwrap();
}

#[cfg(not(feature = "draggable"))]
fn main() {
    println!("Enable draggable feature: cargo run --features draggable");
}
```

## Real-World Applications

### Logo Creation

```rust
use svg_rs::*;

fn create_logo() {
    let mut canvas = Svg::new(200, 200);
    
    // Background circle
    canvas.circle(90)
        .fill("#2c3e50")
        .center(100, 100);
    
    // Inner design
    let group = canvas.group()
        .transform("translate(100, 100)");
    
    // Create geometric pattern
    for i in 0..6 {
        let angle = i as f64 * 60.0;
        let inner_group = group.group()
            .transform(&format!("rotate({})", angle));
        
        inner_group.rect(60, 8)
            .fill("#ecf0f1")
            .move_to(-30, -4);
        
        inner_group.circle(12)
            .fill("#e74c3c")
            .center(25, 0);
    }
    
    canvas.save("logo.svg").unwrap();
}
```

### Icon Set

```rust
use svg_rs::*;

fn create_icon_set() {
    let mut canvas = Svg::new(400, 100);
    
    // Home icon
    let home = canvas.group().transform("translate(50, 50)");
    home.polygon("0,-20 -20,0 -10,0 -10,15 10,15 10,0 20,0").fill("#333");
    home.rect(6, 8).fill("#333").move_to(-3, 7);
    
    // User icon
    let user = canvas.group().transform("translate(150, 50)");
    user.circle(12).fill("#333").center(0, -5);
    user.ellipse(18, 12).fill("#333").center(0, 15);
    
    // Settings icon
    let settings = canvas.group().transform("translate(250, 50)");
    settings.circle(15).fill("none").stroke("#333").stroke_width(3);
    settings.circle(6).fill("#333");
    
    // Search icon
    let search = canvas.group().transform("translate(350, 50)");
    search.circle(12).fill("none").stroke("#333").stroke_width(3);
    search.line(17, 17, 25, 25).stroke("#333").stroke_width(3);
    
    canvas.save("icon_set.svg").unwrap();
}
```

## Running Examples

To run these examples:

1. Create a new Rust project: `cargo new svg_examples`
2. Add svg-rs to Cargo.toml with desired features
3. Copy example code to `src/main.rs`
4. Run with `cargo run`

For feature-specific examples:
```bash
cargo run --features shapes
cargo run --features math
cargo run --features path
cargo run --features draggable
cargo run --features "shapes,math,path"
```
