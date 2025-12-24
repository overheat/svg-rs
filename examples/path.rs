// path.rs
// Demonstrates drawing complex SVG paths. Useful for curves and custom shapes.
// Run with: `cargo run --example path`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut svg = Svg::new(640, 360);

    // A sample quadratic/cubic path
    svg
        .path("M 360 220 Q 400 160 440 220 T 520 220")
        .fill("none")
        .stroke("#073b4c")
        .stroke_width(3);

    svg.save("path.svg")?;
    println!("✅ Saved path.svg");
    Ok(())
}
