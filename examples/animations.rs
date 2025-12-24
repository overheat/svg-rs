// animations.rs
// Demonstrates simple attribute animations (e.g., radius and fill changes).
// Run with: `cargo run --example animations`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut svg = Svg::new(200, 200);

    // Animate radius and color of a circle
    let circle = svg.circle(20).fill("#e63946").center(100, 100);
    circle.animate_attr("r", "20", "80", 3);
    circle.animate_attr("fill", "#e63946", "#457b9d", 2);

    svg.save("animations.svg")?;
    println!("✅ Saved animations.svg");
    Ok(())
}
