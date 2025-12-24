// clip.rs
// Demonstrates `clipPath` usage to restrict rendering to a shape.
// Run with: `cargo run --example clip`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Svg::new(300, 300);
    let defs = canvas.defs();

    // Define a clipPath that will be referenced by id
    let clip = defs.add_child("clipPath", {
        let mut attrs = std::collections::HashMap::new();
        attrs.insert("id".to_string(), "starClip".to_string());
        attrs
    });
    clip.polygon("50,0 61,35 98,35 68,57 79,91 50,70 21,91 32,57 2,35 39,35").fill("white");

    // Apply the clip path to a circle
    canvas.circle(80).fill("#4caf50").center(150, 150).clip_path("starClip");

    canvas.save("clip.svg")?;
    println!("✅ Saved clip.svg");
    Ok(())
}

