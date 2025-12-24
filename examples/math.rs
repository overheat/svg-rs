use svg_rs::*;

#[cfg(feature = "math")]
use svg_rs::math::*;

fn main() {
    #[cfg(feature = "math")]
    {
        let mut canvas = Svg::new(800, 600);
        
        // Example 1: Basic angle calculation and visualization
        let p1 = Point::new(100.0, 100.0);
        let p2 = Point::new(200.0, 150.0);
        let angle = Math::angle(&p1, &p2, None);
        
        println!("Angle between points: {} radians ({} degrees)", 
                 angle, Math::deg(angle));
        
        // Draw the points and line
        canvas.circle(5).fill("#f06").center(p1.x as i32, p1.y as i32);
        canvas.circle(5).fill("#0f6").center(p2.x as i32, p2.y as i32);
        canvas.line(p1.x as i32, p1.y as i32, p2.x as i32, p2.y as i32)
            .stroke("#333").stroke_width(2);
        
        // Example 2: Line operations
        let line = Line::new(Point::new(50.0, 200.0), Point::new(250.0, 300.0));
        let midpoint = line.midpoint();
        
        // Draw the line and its midpoint
        canvas.line(50, 200, 250, 300).stroke("#666").stroke_width(2);
        canvas.circle(5).fill("#f60").center(midpoint.x as i32, midpoint.y as i32);
        
        // Example 3: Perpendicular line
        let perp_line = line.perpendicular_line(&midpoint, 50.0);
        canvas.line(
            perp_line.p1.x as i32, perp_line.p1.y as i32,
            perp_line.p2.x as i32, perp_line.p2.y as i32
        ).stroke("#06f").stroke_width(2);
        
        // Example 4: Linear interpolation visualization
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            let point = line.interpolated_point(t);
            canvas.circle(3).fill("#0f6")
                .center(point.x as i32, point.y as i32);
        }
        
        // Example 5: Angle snapping
        let test_angle = Math::rad(47.0); // 47 degrees
        let directions = [0.0, Math::rad(45.0), Math::rad(90.0), Math::rad(135.0)];
        let snapped = Math::snap_to_angle(test_angle, &directions);
        
        println!("Original angle: {} degrees", Math::deg(test_angle));
        println!("Snapped angle: {} degrees", Math::deg(snapped));
        
        // Example 6: Distance calculation
        let distance = p1.distance_to(&p2);
        println!("Distance between points: {}", distance);
        
        // Add title
        canvas.text("SVG Math Examples")
            .move_to(20, 30)
            .font_size(24)
            .font_family("Arial")
            .fill("#333");
        
        // Add labels
        canvas.text("Angle calculation").move_to(20, 120).font_size(14).fill("#666");
        canvas.text("Line midpoint & perpendicular").move_to(20, 220).font_size(14).fill("#666");
        canvas.text("Linear interpolation points").move_to(20, 320).font_size(14).fill("#666");
        
        // Save the result
        canvas.save("math_examples.svg").unwrap();
        println!("Math examples saved to math_examples.svg");
    }
    
    #[cfg(not(feature = "math"))]
    {
        println!("Math feature not enabled. Run with: cargo run --example math --features math");
    }
}
