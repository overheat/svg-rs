// grouping.rs
// Demonstrates grouping of elements and transformations applied to the group.
// Run with: `cargo run --example grouping`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut svg = Svg::new(200, 200);

    // Create a group with translate+rotate and add shapes inside it
    let group = svg.group().transform("translate(50, 50) rotate(45)");
    group.rect(40, 40).fill("#8e44ad");
    group.circle(20).fill("#f39c12").center(20, 20);

    svg.save("grouping.svg")?;
    println!("✅ Saved grouping.svg");
    Ok(())
}
