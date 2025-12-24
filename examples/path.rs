use svg_rs::*;

#[cfg(feature = "path")]
use svg_rs::path::PathBuilder;

fn main() {
    #[cfg(feature = "path")]
    {
        let mut canvas = Svg::new(800, 600);
        
        // Example 1: Basic path with moveto and lineto
        let path1 = canvas.path("")
            .M(50.0, 50.0)
            .L(150.0, 50.0)
            .L(150.0, 150.0)
            .L(50.0, 150.0)
            .Z();
        path1.fill("#ff6b6b").stroke("#333").stroke_width(2);
        
        // Example 2: Curved path with Bezier curves
        let path2 = canvas.path("")
            .M(200.0, 100.0)
            .C(200.0, 50.0, 300.0, 50.0, 300.0, 100.0)
            .S(400.0, 150.0, 400.0, 100.0);
        path2.fill("none").stroke("#4ecdc4").stroke_width(3);
        
        // Example 3: Quadratic Bezier curves
        let path3 = canvas.path("")
            .M(50.0, 250.0)
            .Q(100.0, 200.0, 150.0, 250.0)
            .T(250.0, 250.0);
        path3.fill("none").stroke("#9b59b6").stroke_width(3);
        
        // Example 4: Arc paths
        let path4 = canvas.path("")
            .M(350.0, 200.0)
            .A(50.0, 30.0, 0.0, 0, 1, 450.0, 250.0)
            .L(400.0, 300.0)
            .Z();
        path4.fill("#f39c12").stroke("#e67e22").stroke_width(2);
        
        // Example 5: Relative path commands
        let path5 = canvas.path("")
            .M(500.0, 100.0)
            .l(50.0, 0.0)
            .l(0.0, 50.0)
            .l(-50.0, 0.0)
            .Z();
        path5.fill("#2ecc71").stroke("#27ae60").stroke_width(2);
        
        // Example 6: Complex path with mixed commands
        let path6 = canvas.path("")
            .M(100.0, 400.0)
            .H(200.0)
            .V(450.0)
            .h(-50.0)
            .v(-25.0)
            .Z();
        path6.fill("#e74c3c").stroke("#c0392b").stroke_width(2);
        
        // Add labels
        canvas.text("Basic Rectangle Path").move_to(50, 30).font_size(14).fill("#333");
        canvas.text("Cubic Bezier Curves").move_to(200, 80).font_size(14).fill("#333");
        canvas.text("Quadratic Bezier").move_to(50, 230).font_size(14).fill("#333");
        canvas.text("Arc Path").move_to(350, 180).font_size(14).fill("#333");
        canvas.text("Relative Commands").move_to(500, 80).font_size(14).fill("#333");
        canvas.text("H/V Commands").move_to(100, 380).font_size(14).fill("#333");
        
        canvas.save("path_examples.svg").unwrap();
        println!("Path examples saved to path_examples.svg");
    }
    
    #[cfg(not(feature = "path"))]
    {
        println!("Path feature not enabled. Run with: cargo run --example path --features path");
    }
}
