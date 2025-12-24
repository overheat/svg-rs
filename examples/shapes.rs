use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(1000, 700);

    // Title
    canvas.text("SVG Shapes Gallery - Stars, Polygons & Cross")
        .move_to(50, 40)
        .font_size(24)
        .font_family("Arial, sans-serif")
        .fill("#2c3e50")
        .font_weight("bold");

    // Row 1: Stars
    canvas.text("Stars").move_to(50, 80).font_size(16).fill("#34495e");
    
    canvas.star(3, 20.0, 40.0).fill("#e74c3c").stroke("#333").stroke_width(2).move_to(100, 100);
    canvas.star(5, 30.0, 60.0).fill("#ff6b6b").stroke("#333").stroke_width(2).move_to(200, 100);
    canvas.star(7, 25.0, 50.0).fill("#4ecdc4").stroke("#2c3e50").stroke_width(1).move_to(320, 100);
    canvas.star(8, 25.0, 45.0).fill("#9b59b6").stroke("#333").stroke_width(2).move_to(440, 100);
    canvas.star(12, 20.0, 40.0).fill("#1abc9c").stroke("#333").stroke_width(1).move_to(560, 100);

    // Row 2: Regular polygons
    canvas.text("Regular Polygons").move_to(50, 240).font_size(16).fill("#34495e");
    
    canvas.ngon(3, 40.0).fill("#e67e22").stroke("#333").stroke_width(2).move_to(100, 260);
    canvas.ngon(4, 35.0).fill("#27ae60").stroke("#333").stroke_width(2).move_to(200, 260);
    canvas.ngon(5, 40.0).fill("#cba6f7").stroke("#333").stroke_width(2).move_to(320, 260);
    canvas.ngon(6, 40.0).fill("#95e1d3").stroke("#2c3e50").stroke_width(2).move_to(440, 260);
    canvas.ngon(8, 35.0).fill("#f38ba8").stroke("#2c3e50").stroke_width(2).move_to(560, 260);
    canvas.ngon(10, 35.0).fill("#89b4fa").stroke("#2c3e50").stroke_width(2).move_to(680, 260);

    // Row 3: Cross shapes and animations
    canvas.text("Cross Shapes & Animations").move_to(50, 400).font_size(16).fill("#34495e");
    
    canvas.cross(60.0, 80.0, 20.0).fill("#ff9500").stroke("#2c3e50").stroke_width(2).move_to(100, 420);
    canvas.cross(80.0, 60.0, 15.0).fill("#e67e22").stroke("#2c3e50").stroke_width(2).move_to(220, 420);
    canvas.cross(100.0, 100.0, 10.0).fill("#2ecc71").stroke("#333").stroke_width(1).move_to(340, 420);

    // Animated star
    let animated_star = canvas.star(5, 30.0, 50.0).fill("#e74c3c").stroke("#c0392b").stroke_width(2).move_to(480, 420);
    animated_star.animate_attr("transform", "translate(50, 50) scale(1)", "translate(50, 50) scale(1.5)", 2);

    // Animated ngon
    let animated_ngon = canvas.ngon(6, 40.0).fill("#3498db").stroke("#2980b9").stroke_width(2).move_to(600, 420);
    animated_ngon.animate_attr("transform", "translate(40, 40) rotate(0)", "translate(40, 40) rotate(360)", 3);

    // Complex composition
    canvas.text("Complex Composition").move_to(50, 560).font_size(16).fill("#34495e");
    
    let group = canvas.group().transform("translate(150, 580)");
    group.star(8, 15.0, 30.0).fill("#f1c40f").stroke("#f39c12").stroke_width(1);
    
    for i in 0..8 {
        let angle = i as f64 * 45.0;
        let x = 50.0 * (angle * std::f64::consts::PI / 180.0).cos();
        let y = 50.0 * (angle * std::f64::consts::PI / 180.0).sin();
        group.circle(8).fill("#e74c3c").stroke("#c0392b").stroke_width(1).center(x as i32, y as i32);
    }

    // Labels
    let labels = [
        ("3-star", 110, 200), ("5-star", 210, 200), ("7-star", 330, 200), ("8-star", 450, 200), ("12-star", 570, 200),
        ("triangle", 110, 360), ("square", 210, 360), ("pentagon", 330, 360), ("hexagon", 450, 360), ("octagon", 570, 360), ("decagon", 690, 360),
        ("cross", 110, 520), ("thin cross", 230, 520), ("thick cross", 350, 520), ("animated", 490, 520), ("rotating", 610, 520),
    ];

    for (label, x, y) in labels.iter() {
        canvas.text(label).move_to(*x, *y).font_size(10).fill("#7f8c8d").text_anchor("middle");
    }

    // Save to file
    canvas.save("shapes_demo.svg").unwrap();
    println!("Comprehensive shapes demo saved to shapes_demo.svg");
}
