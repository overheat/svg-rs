// mask.rs
// Demonstrates creating and using an SVG mask. Masks control pixel visibility
// based on the mask shapes; common for soft edges or complex composites.
// Run with: `cargo run --example mask`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Svg::new(300, 200);
    let defs = canvas.defs();

    // Define a mask with an id and a white-filled circle (white = visible)
    let mask = defs.add_child("mask", {
        let mut attrs = std::collections::HashMap::new();
        attrs.insert("id".to_string(), "circleMask".to_string());
        attrs
    });
    mask.circle(50).fill("white").center(0, 0);

    // Apply mask to a rectangle
    canvas.rect(200, 150).fill("#00bcd4").move_to(50, 25).mask("circleMask");

    canvas.save("mask.svg")?;
    println!("✅ Saved mask.svg");
    Ok(())
}

