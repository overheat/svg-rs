# Shapes Feature

The shapes feature provides advanced geometric shapes beyond basic SVG elements.

## Enabling the Feature

```toml
[dependencies]
svg-rs = { version = "0.2", features = ["shapes"] }
```

## Available Shapes

### Star

Create multi-pointed star shapes:

```rust
use svg_rs::*;

let mut canvas = Svg::new(400, 300);

// 5-pointed star
canvas.star(5, 30.0, 60.0)  // spikes, inner_radius, outer_radius
    .fill("#ff6b6b")
    .stroke("#333")
    .stroke_width(2)
    .move_to(100, 100);

// 8-pointed star
canvas.star(8, 25.0, 50.0)
    .fill("#4ecdc4")
    .move_to(250, 100);
```

#### Parameters
- `spikes`: Number of star points
- `inner_radius`: Distance from center to inner points
- `outer_radius`: Distance from center to outer points

### Regular Polygon (N-gon)

Create regular polygons with any number of sides:

```rust
// Triangle (3 sides)
canvas.ngon(3, 40.0)  // edges, radius
    .fill("#9b59b6")
    .move_to(100, 200);

// Pentagon (5 sides)
canvas.ngon(5, 35.0)
    .fill("#e67e22")
    .move_to(200, 200);

// Hexagon (6 sides)
canvas.ngon(6, 40.0)
    .fill("#2ecc71")
    .move_to(300, 200);

// Octagon (8 sides)
canvas.ngon(8, 35.0)
    .fill("#3498db")
    .move_to(400, 200);
```

#### Parameters
- `edges`: Number of sides (minimum 3)
- `radius`: Distance from center to vertices

### Cross

Create cross shapes with customizable dimensions:

```rust
// Basic cross
canvas.cross(60.0, 80.0, 20.0)  // width, height, thickness
    .fill("#e74c3c")
    .stroke("#c0392b")
    .stroke_width(2)
    .move_to(100, 300);

// Wide cross
canvas.cross(100.0, 60.0, 15.0)
    .fill("#f39c12")
    .move_to(250, 300);
```

#### Parameters
- `width`: Total width of the cross
- `height`: Total height of the cross
- `thickness`: Thickness of the cross arms

## Complete Example

```rust
use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 600);
    
    // Title
    canvas.text("Advanced Shapes Gallery")
        .move_to(20, 30)
        .font_size(24)
        .font_family("Arial")
        .fill("#333");
    
    // Stars row
    canvas.text("Stars").move_to(20, 80).font_size(16).fill("#666");
    
    canvas.star(3, 20.0, 40.0).fill("#e74c3c").move_to(100, 100);
    canvas.star(5, 30.0, 60.0).fill("#ff6b6b").move_to(200, 100);
    canvas.star(7, 25.0, 50.0).fill("#4ecdc4").move_to(320, 100);
    canvas.star(8, 25.0, 45.0).fill("#9b59b6").move_to(440, 100);
    canvas.star(12, 20.0, 40.0).fill("#1abc9c").move_to(560, 100);
    
    // Polygons row
    canvas.text("Regular Polygons").move_to(20, 240).font_size(16).fill("#666");
    
    canvas.ngon(3, 40.0).fill("#e67e22").move_to(100, 260);
    canvas.ngon(4, 35.0).fill("#27ae60").move_to(200, 260);
    canvas.ngon(5, 40.0).fill("#cba6f7").move_to(320, 260);
    canvas.ngon(6, 40.0).fill("#95e1d3").move_to(440, 260);
    canvas.ngon(8, 35.0).fill("#f38ba8").move_to(560, 260);
    canvas.ngon(10, 35.0).fill("#89b4fa").move_to(680, 260);
    
    // Crosses row
    canvas.text("Crosses").move_to(20, 400).font_size(16).fill("#666");
    
    canvas.cross(60.0, 80.0, 20.0).fill("#ff9500").move_to(100, 420);
    canvas.cross(80.0, 60.0, 15.0).fill("#e67e22").move_to(220, 420);
    canvas.cross(100.0, 100.0, 10.0).fill("#2ecc71").move_to(340, 420);
    
    // Animated shapes
    let animated_star = canvas.star(5, 30.0, 50.0)
        .fill("#e74c3c")
        .stroke("#c0392b")
        .stroke_width(2)
        .move_to(480, 420);
    animated_star.animate_attr("transform", "rotate(0)", "rotate(360)", 3);
    
    let animated_ngon = canvas.ngon(6, 40.0)
        .fill("#3498db")
        .stroke("#2980b9")
        .stroke_width(2)
        .move_to(600, 420);
    animated_ngon.animate_attr("transform", "rotate(0)", "rotate(-360)", 4);
    
    // Grouped shapes
    let group = canvas.group()
        .transform("translate(50, 500)");
    
    group.star(8, 15.0, 30.0).fill("#f1c40f").stroke("#f39c12").stroke_width(1);
    group.ngon(4, 20.0).fill("#e74c3c").stroke("#c0392b").stroke_width(1)
        .transform("translate(60, 0)");
    group.cross(40.0, 40.0, 8.0).fill("#9b59b6").stroke("#8e44ad").stroke_width(1)
        .transform("translate(120, 0)");
    
    canvas.save("shapes_gallery.svg").unwrap();
    println!("Shapes gallery saved to shapes_gallery.svg");
}
```

## Styling Shapes

All shapes support the same styling methods as basic SVG elements:

```rust
canvas.star(6, 25.0, 50.0)
    .fill("#ff6b6b")           // Fill color
    .stroke("#333")            // Stroke color
    .stroke_width(2)           // Stroke width
    .opacity(0.8)              // Opacity
    .class("star-shape")       // CSS class
    .rotate(30.0)              // Rotation
    .scale(1.5, 1.0);          // Scaling
```

## Animation

Shapes can be animated like any other SVG element:

```rust
// Rotating star
canvas.star(5, 30.0, 60.0)
    .fill("#e74c3c")
    .move_to(200, 200)
    .animate_attr("transform", "rotate(0)", "rotate(360)", 3);

// Scaling polygon
canvas.ngon(6, 40.0)
    .fill("#3498db")
    .center(300, 200)
    .animate_attr("transform", "scale(1)", "scale(1.5)", 2);
```

## Mathematical Properties

### Star Points Calculation

Stars are generated using trigonometric calculations:
- Outer points at angles: `i * 2π / spikes`
- Inner points at angles: `(i + 0.5) * 2π / spikes`
- Inner radius determines the "sharpness" of the star

### Polygon Vertices

Regular polygons have vertices calculated as:
- Angle for vertex i: `i * 2π / edges`
- Position: `(center_x + radius * cos(angle), center_y + radius * sin(angle))`

### Cross Construction

Crosses are built as polygons with 12 vertices forming the cross shape, allowing for precise control over dimensions and proportions.

## Best Practices

1. **Proportions**: Keep inner/outer radius ratios reasonable for stars (typically 0.3-0.7)
2. **Positioning**: Use `move_to()` for precise positioning
3. **Grouping**: Group related shapes for complex compositions
4. **Animation**: Combine with transforms for dynamic effects
5. **Styling**: Use consistent color schemes and stroke widths

## Performance Notes

- Shapes are generated as SVG polygons
- Complex shapes (many points) may impact rendering performance
- Consider using groups for repeated shape patterns
- Animations are handled by the browser's SVG engine
