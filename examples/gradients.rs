// gradients.rs
// Minimal example showing how to define and apply a linear gradient.
// Run with: `cargo run --example gradients`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut svg = Svg::new(200, 200);

    // Create a defs section and a linear gradient with multiple stops
    let defs = svg.defs();
    let grad = defs.linear_gradient("testGrad");
    grad.stop("0%", "#ff0000");
    grad.stop("50%", "#00ff00");
    grad.stop("100%", "#0000ff");

    // Apply the gradient as a fill via `url(#id)`
    svg.rect(150, 100).fill("url(#testGrad)").move_to(25, 50);

    svg.save("gradients.svg")?;
    println!("✅ Saved gradients.svg");
    Ok(())
}
