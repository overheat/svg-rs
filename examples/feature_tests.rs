use svg_rs::*;

fn main() {
    println!("🧪 Testing SVG-RS Core Features\n");
    
    // 测试基础功能
    test_basic_shapes();
    test_grouping();
    test_gradients();
    test_animations();
    test_events();
    
    println!("✅ All manual tests completed!");
}

fn save(svg: Svg, name: &str) {
    svg
        .save(name)
        .unwrap_or_else(|e| panic!("Failed to save {}: {}", name, e));
    println!("   ✓ Saved {}", name);
}

fn test_basic_shapes() {
    println!("📐 Testing basic shapes...");
    let mut svg = Svg::new(200, 200);
    
    svg.rect(50, 50).fill("#e63946").move_to(10, 10);
    svg.circle(25).fill("#457b9d").center(100, 100);
    svg.polygon("50,10 90,90 10,90").fill("#2a9d8f");
    
    save(svg, "test_shapes.svg");
}

fn test_grouping() {
    println!("👥 Testing grouping...");
    let mut svg = Svg::new(200, 200);
    
    let group = svg.group().transform("translate(50, 50) rotate(45)");
    group.rect(40, 40).fill("#8e44ad");
    group.circle(20).fill("#f39c12").center(20, 20);
    
    save(svg, "test_group.svg");
}

fn test_gradients() {
    println!("🌈 Testing gradients...");
    let mut svg = Svg::new(200, 200);
    
    let defs = svg.defs();
    let grad = defs.linear_gradient("testGrad");
    grad.stop("0%", "#ff0000");
    grad.stop("50%", "#00ff00");
    grad.stop("100%", "#0000ff");
    
    svg.rect(150, 100)
        .fill("url(#testGrad)")
        .move_to(25, 50);
    
    save(svg, "test_gradient.svg");
}

fn test_animations() {
    println!("🎬 Testing animations...");
    let mut svg = Svg::new(200, 200);
    
    let circle = svg.circle(20).fill("#e63946").center(100, 100);
    circle.animate_attr("r", "20", "80", 3);
    circle.animate_attr("fill", "#e63946", "#457b9d", 2);
    
    save(svg, "test_animation.svg");
}

fn test_events() {
    println!("🖱️  Testing events...");
    let mut svg = Svg::new(200, 200);
    
    svg.rect(100, 50)
        .fill("#yellow")
        .move_to(50, 75)
        .on_click("alert('Hello from SVG-RS!')")
        .on_hover("this.style.fill='red'");
    
    save(svg, "test_events.svg");
}
