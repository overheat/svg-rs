// basic_shapes.rs
// Minimal example demonstrating basic shape creation:
// - `rect`, `circle`, `polygon`
// Run with: `cargo run --example basic_shapes`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut svg = Svg::new(200, 200);

    // Draw a rectangle, a circle, and a polygon with simple fills
    svg.rect(50, 50).fill("#e63946").move_to(10, 10);
    svg.circle(25).fill("#457b9d").center(100, 100);
    svg.polygon("50,10 90,90 10,90").fill("#2a9d8f");

    svg.save("basic_shapes.svg")?;
    println!("✅ Saved basic_shapes.svg");
    Ok(())
}
