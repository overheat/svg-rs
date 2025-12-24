// styles.rs
// Demonstrates adding a `<style>` element and using class/inline styles.
// Run with: `cargo run --example styles`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Svg::new(400, 200);

    // Insert CSS rules via a style element
    let _style = canvas.style_element(
        ".highlight { fill: yellow; stroke: red; stroke-width: 2; }\n.fade { opacity: 0.5; }"
    );

    // Use class-based styling and inline `style` attribute examples
    canvas.rect(150, 100).fill("#blue").move_to(20, 20).class("highlight fade");
    canvas.circle(30).center(300, 60).style("fill: orange; stroke: purple; stroke-width: 3;");

    canvas.save("styles.svg")?;
    println!("✅ Saved styles.svg");
    Ok(())
}

