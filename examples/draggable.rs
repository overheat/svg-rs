use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 600);

    // Create a draggable rectangle
    canvas.rect(100, 100)
        .fill("#ff6b6b")
        .move_to(50, 50)
        .id("draggable-rect")
        .draggable();

    // Create a draggable circle with constraints
    canvas.circle(40)
        .fill("#4ecdc4")
        .center(200, 150)
        .id("constrained-circle")
        .draggable()
        .drag_constrain(100.0, 100.0, 400.0, 200.0);

    // Create a draggable element with grid snapping
    canvas.rect(60, 60)
        .fill("#45b7d1")
        .move_to(400, 200)
        .id("grid-snap-rect")
        .draggable()
        .drag_snap_grid(25.0);

    // Create a regular non-draggable element for comparison
    canvas.circle(30)
        .fill("#96ceb4")
        .center(600, 100);

    // Add some instructions
    canvas.text("Drag the colored shapes!")
        .move_to(50, 30)
        .font_family("Arial")
        .font_size(18)
        .fill("#333");

    canvas.text("Red rectangle: free drag")
        .move_to(50, 500)
        .font_family("Arial")
        .font_size(14)
        .fill("#666");

    canvas.text("Teal circle: constrained to box")
        .move_to(50, 520)
        .font_family("Arial")
        .font_size(14)
        .fill("#666");

    canvas.text("Blue rectangle: snaps to 25px grid")
        .move_to(50, 540)
        .font_family("Arial")
        .font_size(14)
        .fill("#666");

    // Save to file
    canvas.save("draggable_demo.svg").unwrap();
    println!("Draggable demo saved to draggable_demo.svg");
    println!("Open in a web browser to test the drag functionality!");
}
