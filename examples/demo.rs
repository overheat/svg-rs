use svg_rs::*;

fn main() {
    // 创建一个复杂的 SVG 示例
    let mut canvas = Svg::new(600, 400);
    
    // 背景矩形
    canvas.rect(600, 400)
        .fill("#f8f9fa")
        .move_to(0, 0);
    
    // 标题文本
    canvas.text("SVG-RS Demo")
        .move_to(250, 30)
        .fill("#212529");
    
    // 彩色矩形组
    canvas.rect(80, 60)
        .fill("#e74c3c")
        .move_to(50, 60);
    
    canvas.rect(80, 60)
        .fill("#3498db")
        .move_to(150, 60);
    
    canvas.rect(80, 60)
        .fill("#2ecc71")
        .move_to(250, 60);
    
    // 圆形组
    canvas.circle(30)
        .fill("#f39c12")
        .center(90, 180);
    
    canvas.circle(30)
        .fill("#9b59b6")
        .center(190, 180);
    
    canvas.circle(30)
        .fill("#1abc9c")
        .center(290, 180);
    
    // 椭圆
    canvas.ellipse(60, 30)
        .fill("#e67e22")
        .center(450, 100);
    
    // 线条装饰
    canvas.line(50, 250, 350, 250)
        .stroke("#34495e")
        .stroke_width(2);
    
    // 路径 - 波浪线
    canvas.path("M 50 280 Q 100 260 150 280 T 250 280 T 350 280")
        .fill("none")
        .stroke("#e74c3c")
        .stroke_width(3);
    
    // 旋转的矩形
    canvas.rect(40, 40)
        .fill("#f1c40f")
        .move_to(450, 200)
        .rotate(45.0);
    
    // 描边圆形
    canvas.circle(25)
        .fill("none")
        .stroke("#2c3e50")
        .stroke_width(3)
        .center(500, 300);
    
    // 半透明元素（如果支持的话）
    canvas.rect(100, 50)
        .fill("#8e44ad")
        .move_to(400, 320);
    
    println!("Generated SVG:");
    println!("{}", canvas.to_string());
    
    // 保存到文件
    match canvas.save("demo.svg") {
        Ok(_) => println!("\n✅ SVG saved to demo.svg"),
        Err(e) => println!("❌ Failed to save: {}", e),
    }
}
