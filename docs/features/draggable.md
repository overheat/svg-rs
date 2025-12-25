# Draggable Feature

The draggable feature provides interactive drag functionality for SVG elements.

## Enabling the Feature

```toml
[dependencies]
svg-rs = { version = "0.2", features = ["draggable"] }
```

```rust
use svg_rs::*;
```

## Basic Draggable Elements

### Making Elements Draggable

```rust
let mut canvas = Svg::new(800, 600);

// Basic draggable rectangle
canvas.rect(100, 100)
    .fill("#ff6b6b")
    .move_to(50, 50)
    .id("draggable-rect")
    .draggable();

// Draggable circle
canvas.circle(40)
    .fill("#4ecdc4")
    .center(200, 150)
    .id("draggable-circle")
    .draggable();
```

### Element Requirements

For draggable functionality to work properly:

1. **ID Required**: Each draggable element must have a unique ID
2. **JavaScript Integration**: The generated SVG includes JavaScript for drag handling
3. **Browser Support**: Requires modern browsers with SVG and JavaScript support

## Drag Constraints

### Boundary Constraints

Limit dragging to specific rectangular areas:

```rust
// Constrain to specific bounds (x, y, width, height)
canvas.rect(60, 60)
    .fill("#45b7d1")
    .move_to(100, 100)
    .id("constrained-rect")
    .draggable()
    .drag_constrain(50.0, 50.0, 300.0, 200.0);
    // Can only be dragged within x:50-350, y:50-250
```

### Grid Snapping

Snap elements to grid positions during drag:

```rust
// Snap to 25-pixel grid
canvas.circle(30)
    .fill("#e74c3c")
    .center(200, 200)
    .id("grid-snap-circle")
    .draggable()
    .drag_snap_grid(25.0);
```

### Combined Constraints

```rust
// Both boundary constraints and grid snapping
canvas.rect(80, 80)
    .fill("#9b59b6")
    .move_to(300, 100)
    .id("constrained-grid-rect")
    .draggable()
    .drag_constrain(250.0, 50.0, 400.0, 300.0)
    .drag_snap_grid(20.0);
```

## Complete Example

```rust
use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 600);
    
    // Title
    canvas.text("Interactive Draggable Elements")
        .move_to(20, 30)
        .font_size(24)
        .font_family("Arial")
        .fill("#333");
    
    // Instructions
    canvas.text("Click and drag the elements below:")
        .move_to(20, 60)
        .font_size(14)
        .fill("#666");
    
    // Free-moving rectangle
    canvas.rect(100, 100)
        .fill("#ff6b6b")
        .stroke("#333")
        .stroke_width(2)
        .move_to(50, 100)
        .id("free-rect")
        .draggable();
    
    canvas.text("Free movement")
        .move_to(50, 220)
        .font_size(12)
        .fill("#666");
    
    // Constrained circle
    canvas.rect(200, 150)  // Visual boundary
        .fill("none")
        .stroke("#ddd")
        .stroke_width(1)
        .stroke_dasharray("5,5")
        .move_to(250, 100);
    
    canvas.circle(40)
        .fill("#4ecdc4")
        .center(350, 175)
        .id("constrained-circle")
        .draggable()
        .drag_constrain(250.0, 100.0, 200.0, 150.0);
    
    canvas.text("Constrained to box")
        .move_to(280, 270)
        .font_size(12)
        .fill("#666");
    
    // Grid-snapped rectangle
    // Draw grid background
    for x in (500..700).step_by(25) {
        canvas.line(x, 100, x, 250)
            .stroke("#f0f0f0")
            .stroke_width(1);
    }
    for y in (100..250).step_by(25) {
        canvas.line(500, y, 700, y)
            .stroke("#f0f0f0")
            .stroke_width(1);
    }
    
    canvas.rect(60, 60)
        .fill("#45b7d1")
        .move_to(525, 125)
        .id("grid-snap-rect")
        .draggable()
        .drag_snap_grid(25.0);
    
    canvas.text("Grid snapping (25px)")
        .move_to(520, 270)
        .font_size(12)
        .fill("#666");
    
    // Combined constraints and grid
    canvas.rect(150, 100)  // Visual boundary
        .fill("none")
        .stroke("#ddd")
        .stroke_width(1)
        .stroke_dasharray("5,5")
        .move_to(100, 350);
    
    // Draw grid within boundary
    for x in (100..250).step_by(20) {
        canvas.line(x, 350, x, 450)
            .stroke("#f8f8f8")
            .stroke_width(1);
    }
    for y in (350..450).step_by(20) {
        canvas.line(100, y, 250, y)
            .stroke("#f8f8f8")
            .stroke_width(1);
    }
    
    canvas.circle(25)
        .fill("#e74c3c")
        .center(175, 400)
        .id("constrained-grid-circle")
        .draggable()
        .drag_constrain(100.0, 350.0, 150.0, 100.0)
        .drag_snap_grid(20.0);
    
    canvas.text("Constrained + Grid (20px)")
        .move_to(100, 470)
        .font_size(12)
        .fill("#666");
    
    // Draggable group
    let group = canvas.group()
        .id("draggable-group")
        .draggable();
    
    group.rect(80, 50)
        .fill("#2ecc71")
        .move_to(400, 350);
    
    group.circle(20)
        .fill("#27ae60")
        .center(440, 375);
    
    group.text("Group")
        .move_to(415, 385)
        .font_size(12)
        .fill("white");
    
    canvas.text("Draggable group")
        .move_to(400, 420)
        .font_size(12)
        .fill("#666");
    
    // Multiple elements with different behaviors
    let colors = ["#f39c12", "#8e44ad", "#16a085", "#c0392b"];
    let positions = [(550, 350), (600, 380), (650, 350), (700, 380)];
    
    for (i, (&(x, y), &color)) in positions.iter().zip(colors.iter()).enumerate() {
        canvas.circle(15)
            .fill(color)
            .center(x, y)
            .id(&format!("multi-circle-{}", i))
            .draggable()
            .drag_snap_grid(10.0);
    }
    
    canvas.text("Multiple draggable elements")
        .move_to(550, 420)
        .font_size(12)
        .fill("#666");
    
    canvas.save("draggable_demo.svg").unwrap();
    println!("Draggable demo saved to draggable_demo.svg");
}
```

## Advanced Draggable Features

### Custom Drag Handlers

While the basic draggable functionality is built-in, you can customize behavior:

```rust
// The draggable feature automatically generates JavaScript
// for handling mouse/touch events and updating element positions
```

### Drag Events

The generated JavaScript handles:

- **Mouse Events**: mousedown, mousemove, mouseup
- **Touch Events**: touchstart, touchmove, touchend (for mobile)
- **Constraint Checking**: Boundary and grid validation
- **Position Updates**: Real-time element positioning

### Performance Considerations

```rust
// For better performance with many draggable elements:

// 1. Use unique IDs
canvas.rect(50, 50)
    .id("unique-id-1")  // Always use unique IDs
    .draggable();

// 2. Minimize constraint calculations
canvas.circle(25)
    .id("simple-drag")
    .draggable();  // No constraints = better performance

// 3. Group related elements
let group = canvas.group()
    .id("group-drag")
    .draggable();  // Drag entire group instead of individual elements
```

## Browser Compatibility

### Supported Browsers

- ✅ Chrome 60+
- ✅ Firefox 55+
- ✅ Safari 12+
- ✅ Edge 79+

### Mobile Support

- ✅ iOS Safari 12+
- ✅ Chrome Mobile 60+
- ✅ Firefox Mobile 55+

### Feature Detection

The generated JavaScript includes feature detection:

```javascript
// Automatically detects touch vs mouse events
// Falls back gracefully on older browsers
// Provides console warnings for unsupported features
```

## Generated JavaScript

The draggable feature automatically includes JavaScript in the SVG:

```xml
<script type="text/javascript">
<![CDATA[
// Drag handling code is automatically generated
// Includes:
// - Event listeners for mouse/touch
// - Constraint validation
// - Grid snapping logic
// - Position updates
]]>
</script>
```

## Styling Draggable Elements

### Visual Feedback

```rust
// Add visual feedback for draggable elements
canvas.rect(100, 100)
    .fill("#3498db")
    .stroke("#2980b9")
    .stroke_width(2)
    .style("cursor: move;")  // CSS cursor hint
    .class("draggable-element")
    .id("styled-drag")
    .draggable();
```

### CSS Integration

```rust
// Add CSS for enhanced styling
canvas.style_element("
    .draggable-element {
        cursor: move;
        transition: opacity 0.2s;
    }
    .draggable-element:hover {
        opacity: 0.8;
    }
    .dragging {
        opacity: 0.6;
        filter: drop-shadow(2px 2px 4px rgba(0,0,0,0.3));
    }
");
```

## Limitations and Considerations

### Current Limitations

1. **JavaScript Dependency**: Requires JavaScript-enabled browsers
2. **SVG Context**: Only works within SVG elements
3. **No Collision Detection**: Elements can overlap during drag
4. **Basic Constraints**: Only rectangular boundaries and grid snapping

### Best Practices

1. **Unique IDs**: Always provide unique IDs for draggable elements
2. **Reasonable Constraints**: Don't make constraint areas too small
3. **Visual Feedback**: Provide clear indication that elements are draggable
4. **Mobile Testing**: Test on touch devices for mobile compatibility
5. **Performance**: Limit the number of simultaneously draggable elements

### Future Enhancements

Potential future features:
- Collision detection
- Custom constraint shapes
- Drag event callbacks
- Animation during drag
- Multi-touch support
- Drag and drop between containers

## Troubleshooting

### Common Issues

1. **Element not dragging**: Check that ID is set and unique
2. **Constraints not working**: Verify constraint bounds are reasonable
3. **Grid snapping issues**: Ensure grid size is appropriate for element size
4. **Mobile not working**: Test touch event support

### Debug Tips

```rust
// Add debug information
canvas.text(&format!("Element ID: {}", "my-element"))
    .move_to(10, 10)
    .font_size(10)
    .fill("#999");

// Visual constraint boundaries
canvas.rect(width, height)
    .fill("none")
    .stroke("#ff0000")
    .stroke_width(1)
    .stroke_dasharray("2,2")
    .move_to(constraint_x, constraint_y);
```
