// polygon_polyline.rs
// Shows usage of `polygon` and `polyline` for multi-point shapes.
// Run with: `cargo run --example polygon_polyline`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Svg::new(600, 400);

    // A filled polygon and an outlined polyline
    canvas.polygon("300,250 350,200 400,250 375,300 325,300").fill("#8e44ad");
    canvas.polyline("450,250 500,200 550,250 500,300 450,350").fill("none").stroke("#ff3b30").stroke_width(4);

    canvas.save("polygon_polyline.svg")?;
    println!("✅ Saved polygon_polyline.svg");
    Ok(())
}

