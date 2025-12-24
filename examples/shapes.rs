// shapes.rs
// 基础几何示例：rect、circle、polygon、polyline。
// 运行：`cargo run --example shapes`

use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut svg = Svg::new(320, 200);

    // rectangle + circle
    svg.rect(80, 60).fill("#e63946").move_to(16, 20);
    svg.circle(30).fill("#457b9d").center(140, 80);

    // polygon + polyline
    svg.polygon("210,40 250,100 170,100").fill("#2a9d8f");
    svg.polyline("40,150 110,140 140,170 190,150 240,180").fill("none").stroke("#ff9f1c").stroke_width(3);

    svg.save("shapes.svg")?;
    println!("✅ Saved shapes.svg");
    Ok(())
}
