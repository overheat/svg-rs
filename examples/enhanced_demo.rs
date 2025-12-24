use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 600);
    
    // 设置视口
    canvas.viewbox(0.0, 0.0, 800.0, 600.0);
    
    // 添加样式
    canvas.style_element("
        .title { font-family: Arial, sans-serif; font-size: 24px; font-weight: bold; }
        .highlight { stroke: #ff0000; stroke-width: 3; }
    ");
    
    // 标题
    canvas.text("SVG-RS Enhanced Features Demo")
        .move_to(400, 40)
        .text_anchor("middle")
        .class("title");
    
    // 基础形状
    canvas.rect(80, 60)
        .fill("#4ecdc4")
        .move_to(50, 100)
        .class("highlight");
    
    canvas.circle(40)
        .fill("#ff6b6b")
        .center(200, 130)
        .on_click("alert('Circle clicked!')");
    
    // 多边形
    canvas.polygon("300,100 330,140 300,180 270,140")
        .fill("#f39c12");
    
    // 折线
    canvas.polyline("400,100 430,120 460,100 490,140 520,120")
        .fill("none")
        .stroke("#45b7d1")
        .stroke_width(3);
    
    // 图像
    canvas.image("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIHZpZXdCb3g9IjAgMCA0MCA0MCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPGNpcmNsZSBjeD0iMjAiIGN5PSIyMCIgcj0iMjAiIGZpbGw9IiNGRjAwMDAiLz4KPC9zdmc+", 40, 40)
        .move_to(600, 100);
    
    // 文本处理
    let text_elem = canvas.text("")
        .move_to(50, 250)
        .font_family("Georgia, serif")
        .font_size(18);
    
    // 变换演示
    canvas.rect(60, 40)
        .fill("#e67e22")
        .move_to(50, 300)
        .skew_x(15.0);
    
    canvas.rect(60, 40)
        .fill("#d35400")
        .move_to(150, 300)
        .flip("x");
    
    // CSS类管理
    let interactive_rect = canvas.rect(120, 40)
        .fill("#2ecc71")
        .move_to(50, 400)
        .class("interactive")
        .add_class("highlight");
    
    // 动画
    let animated_circle = canvas.circle(20)
        .fill("#e74c3c")
        .center(300, 420);
    animated_circle.animate_attr("r", "20", "40", 2);
    
    // 外部对象
    canvas.foreign_object(100, 50)
        .move_to(400, 400);
    
    // 链接
    let link = canvas.link("https://github.com/your-repo/svg-rs");
    link.text("Visit GitHub")
        .move_to(550, 420)
        .fill("#0366d6");
    
    println!("{}", canvas.to_string());
    canvas.save("enhanced_demo.svg").expect("Failed to save");
    println!("✅ Enhanced features demo saved to enhanced_demo.svg");
}
