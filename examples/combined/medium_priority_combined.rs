use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 600);
    
    // 添加CSS样式定义
    let _style_elem = canvas.style_element(
        ".highlight { fill: yellow; stroke: red; stroke-width: 2; }\n         .fade { opacity: 0.5; }\n         .rotate { transform-origin: center; }"
    );
    
    // 创建遮罩定义
    let defs = canvas.defs();
    
    // 创建圆形遮罩
    let mask = defs.add_child("mask", {
        let mut attrs = std::collections::HashMap::new();
        attrs.insert("id".to_string(), "circleMask".to_string());
        attrs
    });
    mask.circle(50).fill("white").center(0, 0);
    
    // 创建裁剪路径
    let clip = defs.add_child("clipPath", {
        let mut attrs = std::collections::HashMap::new();
        attrs.insert("id".to_string(), "starClip".to_string());
        attrs
    });
    clip.polygon("50,0 61,35 98,35 68,57 79,91 50,70 21,91 32,57 2,35 39,35")
        .fill("white");
    
    // 创建渐变
    let gradient = defs.linear_gradient("rainbow");
    gradient.stop("0%", "#ff0000");
    gradient.stop("16%", "#ff8000");
    gradient.stop("33%", "#ffff00");
    gradient.stop("50%", "#00ff00");
    gradient.stop("66%", "#0080ff");
    gradient.stop("83%", "#8000ff");
    gradient.stop("100%", "#ff0080");
    
    // 使用CSS类的矩形
    canvas.rect(150, 100)
        .fill("#blue")
        .move_to(50, 50)
        .class("highlight fade")
        .id("styled-rect");
    
    // 使用内联样式的圆形
    canvas.circle(40)
        .center(300, 100)
        .style("fill: orange; stroke: purple; stroke-width: 3; stroke-dasharray: 5,5");
    
    // 使用遮罩的矩形
    canvas.rect(200, 150)
        .fill("url(#rainbow)")
        .move_to(450, 50)
        .mask("circleMask")
        .transform("translate(50, 50)");
    
    // 使用裁剪路径的圆形
    canvas.circle(80)
        .fill("#green")
        .center(150, 300)
        .clip_path("starClip");
    
    // 多边形和折线示例
    canvas.polygon("300,250 350,200 400,250 375,300 325,300")
        .fill("#purple")
        .add_class("rotate");
    
    canvas.polyline("450,250 500,200 550,250 500,300 450,350")
        .fill("none")
        .stroke("#red")
        .stroke_width(4)
        .add_class("highlight");
    
    // 带动画和事件的组合元素
    let interactive_group = canvas.group()
        .transform("translate(100, 450)")
        .class("interactive")
        .on_click("this.style.transform += ' scale(1.2)'")
        .on_hover("this.style.opacity = '0.8'");
    
    let animated_rect = interactive_group.rect(80, 60)
        .fill("#cyan")
        .move_to(0, 0);
    animated_rect.animate_attr("width", "80", "120", 2);
    
    interactive_group.text("Click me!")
        .move_to(10, 35)
        .fill("#black")
        .style("font-family: Arial; font-size: 12px;");
    
    println!("{}", canvas.to_string());
    canvas.save("medium_priority_demo.svg").expect("Failed to save");
    println!("✅ Medium priority features demo saved to medium_priority_demo.svg");
}
