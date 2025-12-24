use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 600);
    
    // 创建渐变定义
    let defs = canvas.defs();
    let gradient = defs.linear_gradient("myGradient");
    gradient.stop("0%", "#ff0000");
    gradient.stop("100%", "#0000ff");
    
    // 使用渐变的矩形
    canvas.rect(200, 100)
        .fill("url(#myGradient)")
        .move_to(50, 50);
    
    // 分组元素
    let group = canvas.group()
        .transform("translate(300, 100)");
    
    group.circle(30)
        .fill("#00ff00")
        .center(0, 0);
    
    group.rect(40, 40)
        .fill("#ff00ff")
        .move_to(-20, 40);
    
    // 带动画的圆形
    let animated_circle = canvas.circle(25)
        .fill("#ffff00")
        .center(400, 300);
    
    animated_circle.animate_attr("r", "25", "50", 2);
    animated_circle.animate_attr("fill", "#ffff00", "#ff0000", 3);
    
    // 交互元素
    canvas.rect(100, 50)
        .fill("#00ffff")
        .move_to(500, 200)
        .on_click("alert('Clicked!')")
        .on_hover("this.style.opacity='0.5'");
    
    // 多边形
    canvas.polygon("100,10 40,198 190,78 10,78 160,198")
        .fill("#ffa500")
        .transform("translate(200, 400) scale(0.5)");
    
    // 折线
    canvas.polyline("20,20 40,25 60,40 80,120 120,140 200,180")
        .fill("none")
        .stroke("#000000")
        .stroke_width(3)
        .transform("translate(400, 400)");
    
    println!("{}", canvas.to_string());
    canvas.save("advanced_demo.svg").expect("Failed to save");
    println!("✅ Advanced demo saved to advanced_demo.svg");
}
