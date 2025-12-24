// events.rs
// Shows how to attach simple event handlers (`on_click`, `on_hover`) to elements.
// Note: browser environments interpret these handlers; viewing the SVG in a browser
// will demonstrate the interactive behavior.
// Run with: `cargo run --example events`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut svg = Svg::new(200, 200);

    // Rectangle with click and hover handlers
    svg.rect(100, 50)
        .fill("#ffee58")
        .move_to(50, 75)
        .on_click("alert('Hello from SVG-RS!')")
        .on_hover("this.style.fill='red'");

    svg.save("events.svg")?;
    println!("✅ Saved events.svg");
    Ok(())
}
