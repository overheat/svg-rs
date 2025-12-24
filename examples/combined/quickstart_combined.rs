use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Svg::new(360, 220);

    canvas.rect(360, 220).fill("#f5f7fb");

    canvas
        .text("SVG-RS Demo")
        .move_to(120, 32)
        .fill("#161616");

    canvas
        .circle(36)
        .fill("#ff6b6b")
        .stroke("#222222")
        .stroke_width(2)
        .center(90, 120);

    canvas
        .rect(120, 60)
        .fill("none")
        .stroke("#1e90ff")
        .stroke_width(3)
        .move_to(170, 80);

    canvas
        .line(40, 180, 320, 180)
        .stroke("#7a869a")
        .stroke_width(2);

    canvas.save("quickstart.svg")?;
    println!("✅ Saved quickstart.svg");
    Ok(())
}
