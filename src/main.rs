use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 600);
    
    // Create rectangle
    canvas.rect(100, 100)
        .fill("#f06")
        .move_to(50, 50);
    
    // Create circle
    canvas.circle(50)
        .fill("#0f6")
        .center(200, 100);
    
    // Create ellipse
    canvas.ellipse(80, 40)
        .fill("#06f")
        .center(350, 100);
    
    // Create line
    canvas.line(50, 200, 200, 250)
        .stroke("#000")
        .stroke_width(3);
    
    // Create path
    canvas.path("M 250 200 Q 300 150 350 200 T 450 200")
        .fill("none")
        .stroke("#f60")
        .stroke_width(2);
    
    // Create text
    canvas.text("Hello SVG-RS!")
        .move_to(50, 300)
        .fill("#333");
    
    // Create transformed rectangle
    canvas.rect(60, 60)
        .fill("#f0f")
        .move_to(400, 250)
        .rotate(45.0);
    
    println!("{}", canvas.to_string());
    
    // Save to file
    canvas.save("example.svg").expect("Failed to save SVG file");
}
