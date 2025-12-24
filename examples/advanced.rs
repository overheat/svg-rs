use svg_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Svg::new(640, 360);

    // 背景渐变
    let defs = canvas.defs();
    let gradient = defs.linear_gradient("sunset");
    gradient.stop("0%", "#ff9a9e");
    gradient.stop("100%", "#fad0c4");
    canvas.rect(640, 360).fill("url(#sunset)");

    // 分组 + 变换
    let group = canvas.group().transform("translate(80, 60)");
    group.rect(200, 120).fill("#ffffff").stroke("#222222").stroke_width(2);
    group.circle(30).fill("#2d9cdb").center(60, 60);
    group.circle(30).fill("#27ae60").center(140, 60);

    // 路径示例
    canvas
        .path("M 360 220 Q 400 160 440 220 T 520 220")
        .fill("none")
        .stroke("#073b4c")
        .stroke_width(3);

    // 动画圆
    let animated = canvas.circle(20).fill("#ffd166").center(420, 140);
    animated.animate_attr("cx", "420", "520", 2);
    animated.animate_attr("r", "20", "32", 2);

    // 交互按钮
    canvas
        .rect(120, 50)
        .fill("#ef476f")
        .move_to(420, 240)
        .on_click("alert('Clicked!')")
        .on_hover("this.style.opacity='0.6'");

    canvas.save("advanced_showcase.svg")?;
    println!("✅ Saved advanced_showcase.svg");
    Ok(())
}
