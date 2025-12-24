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

fn test_basic_shapes() {
    println!("📐 Testing basic shapes...");
    let mut svg = Svg::new(200, 200);
    
    svg.rect(50, 50).fill("#red").move_to(10, 10);
    svg.circle(25).fill("#blue").center(100, 100);
    svg.polygon("50,10 90,90 10,90").fill("#green");
    
    svg.save("test_shapes.svg").unwrap();
    println!("   ✓ Shapes saved to test_shapes.svg");
}

fn test_grouping() {
    println!("👥 Testing grouping...");
    let mut svg = Svg::new(200, 200);
    
    let group = svg.group().transform("translate(50, 50) rotate(45)");
    group.rect(40, 40).fill("#purple");
    group.circle(20).fill("#orange").center(20, 20);
    
    svg.save("test_group.svg").unwrap();
    println!("   ✓ Group saved to test_group.svg");
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
    
    svg.save("test_gradient.svg").unwrap();
    println!("   ✓ Gradient saved to test_gradient.svg");
}

fn test_animations() {
    println!("🎬 Testing animations...");
    let mut svg = Svg::new(200, 200);
    
    let circle = svg.circle(20).fill("#red").center(100, 100);
    circle.animate_attr("r", "20", "80", 3);
    circle.animate_attr("fill", "#red", "#blue", 2);
    
    svg.save("test_animation.svg").unwrap();
    println!("   ✓ Animation saved to test_animation.svg");
}

fn test_events() {
    println!("🖱️  Testing events...");
    let mut svg = Svg::new(200, 200);
    
    svg.rect(100, 50)
        .fill("#yellow")
        .move_to(50, 75)
        .on_click("alert('Hello from SVG-RS!')")
        .on_hover("this.style.fill='red'");
    
    svg.save("test_events.svg").unwrap();
    println!("   ✓ Events saved to test_events.svg");
}
