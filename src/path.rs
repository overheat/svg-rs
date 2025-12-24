use crate::svg::Element;

/// Path segment types for SVG path construction
#[derive(Debug, Clone)]
pub struct PathSegment {
    pub command: String,
    pub coords: Vec<f64>,
}

/// Enhanced path builder with fluent API
pub trait PathBuilder {
    /// Move to absolute position
    fn M(&mut self, x: f64, y: f64) -> &mut Self;
    /// Move to relative position  
    fn m(&mut self, dx: f64, dy: f64) -> &mut Self;
    /// Line to absolute position
    fn L(&mut self, x: f64, y: f64) -> &mut Self;
    /// Line to relative position
    fn l(&mut self, dx: f64, dy: f64) -> &mut Self;
    /// Horizontal line to absolute x
    fn H(&mut self, x: f64) -> &mut Self;
    /// Horizontal line relative
    fn h(&mut self, dx: f64) -> &mut Self;
    /// Vertical line to absolute y
    fn V(&mut self, y: f64) -> &mut Self;
    /// Vertical line relative
    fn v(&mut self, dy: f64) -> &mut Self;
    /// Cubic Bezier curve absolute
    fn C(&mut self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, x: f64, y: f64) -> &mut Self;
    /// Cubic Bezier curve relative
    fn c(&mut self, dc1x: f64, dc1y: f64, dc2x: f64, dc2y: f64, dx: f64, dy: f64) -> &mut Self;
    /// Smooth cubic Bezier absolute
    fn S(&mut self, c2x: f64, c2y: f64, x: f64, y: f64) -> &mut Self;
    /// Smooth cubic Bezier relative
    fn s(&mut self, dc2x: f64, dc2y: f64, dx: f64, dy: f64) -> &mut Self;
    /// Quadratic Bezier curve absolute
    fn Q(&mut self, cx: f64, cy: f64, x: f64, y: f64) -> &mut Self;
    /// Quadratic Bezier curve relative
    fn q(&mut self, dcx: f64, dcy: f64, dx: f64, dy: f64) -> &mut Self;
    /// Smooth quadratic Bezier absolute
    fn T(&mut self, x: f64, y: f64) -> &mut Self;
    /// Smooth quadratic Bezier relative
    fn t(&mut self, dx: f64, dy: f64) -> &mut Self;
    /// Arc absolute
    fn A(&mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc_flag: i32, sweep_flag: i32, x: f64, y: f64) -> &mut Self;
    /// Arc relative
    fn a(&mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc_flag: i32, sweep_flag: i32, dx: f64, dy: f64) -> &mut Self;
    /// Close path
    fn Z(&mut self) -> &mut Self;
    
    /// Utility methods
    fn clear_path(&mut self) -> &mut Self;
    fn get_segment_count(&self) -> usize;
    fn get_segment(&self, index: usize) -> Option<&PathSegment>;
    fn remove_segment(&mut self, index: usize) -> &mut Self;
    fn replace_segment(&mut self, index: usize, segment: PathSegment) -> &mut Self;
    fn redraw(&mut self) -> &mut Self;
}

impl PathBuilder for Element {
    fn M(&mut self, x: f64, y: f64) -> &mut Self {
        self.add_segment("M", vec![x, y])
    }

    fn m(&mut self, dx: f64, dy: f64) -> &mut Self {
        self.add_segment("m", vec![dx, dy])
    }

    fn L(&mut self, x: f64, y: f64) -> &mut Self {
        self.add_segment("L", vec![x, y])
    }

    fn l(&mut self, dx: f64, dy: f64) -> &mut Self {
        self.add_segment("l", vec![dx, dy])
    }

    fn H(&mut self, x: f64) -> &mut Self {
        self.add_segment("H", vec![x])
    }

    fn h(&mut self, dx: f64) -> &mut Self {
        self.add_segment("h", vec![dx])
    }

    fn V(&mut self, y: f64) -> &mut Self {
        self.add_segment("V", vec![y])
    }

    fn v(&mut self, dy: f64) -> &mut Self {
        self.add_segment("v", vec![dy])
    }

    fn C(&mut self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, x: f64, y: f64) -> &mut Self {
        self.add_segment("C", vec![c1x, c1y, c2x, c2y, x, y])
    }

    fn c(&mut self, dc1x: f64, dc1y: f64, dc2x: f64, dc2y: f64, dx: f64, dy: f64) -> &mut Self {
        self.add_segment("c", vec![dc1x, dc1y, dc2x, dc2y, dx, dy])
    }

    fn S(&mut self, c2x: f64, c2y: f64, x: f64, y: f64) -> &mut Self {
        self.add_segment("S", vec![c2x, c2y, x, y])
    }

    fn s(&mut self, dc2x: f64, dc2y: f64, dx: f64, dy: f64) -> &mut Self {
        self.add_segment("s", vec![dc2x, dc2y, dx, dy])
    }

    fn Q(&mut self, cx: f64, cy: f64, x: f64, y: f64) -> &mut Self {
        self.add_segment("Q", vec![cx, cy, x, y])
    }

    fn q(&mut self, dcx: f64, dcy: f64, dx: f64, dy: f64) -> &mut Self {
        self.add_segment("q", vec![dcx, dcy, dx, dy])
    }

    fn T(&mut self, x: f64, y: f64) -> &mut Self {
        self.add_segment("T", vec![x, y])
    }

    fn t(&mut self, dx: f64, dy: f64) -> &mut Self {
        self.add_segment("t", vec![dx, dy])
    }

    fn A(&mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc_flag: i32, sweep_flag: i32, x: f64, y: f64) -> &mut Self {
        self.add_segment("A", vec![rx, ry, x_axis_rotation, large_arc_flag as f64, sweep_flag as f64, x, y])
    }

    fn a(&mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc_flag: i32, sweep_flag: i32, dx: f64, dy: f64) -> &mut Self {
        self.add_segment("a", vec![rx, ry, x_axis_rotation, large_arc_flag as f64, sweep_flag as f64, dx, dy])
    }

    fn Z(&mut self) -> &mut Self {
        self.add_segment("Z", vec![])
    }

    fn clear_path(&mut self) -> &mut Self {
        if let Some(segments) = self.path_segments.as_mut() {
            segments.clear();
        }
        self.set_attr("d", "")
    }

    fn get_segment_count(&self) -> usize {
        self.path_segments.as_ref().map_or(0, |s| s.len())
    }

    fn get_segment(&self, index: usize) -> Option<&PathSegment> {
        self.path_segments.as_ref()?.get(index)
    }

    fn remove_segment(&mut self, index: usize) -> &mut Self {
        if let Some(segments) = self.path_segments.as_mut() {
            if index < segments.len() {
                segments.remove(index);
                self.redraw();
            }
        }
        self
    }

    fn replace_segment(&mut self, index: usize, segment: PathSegment) -> &mut Self {
        if let Some(segments) = self.path_segments.as_mut() {
            if index < segments.len() {
                segments[index] = segment;
                self.redraw();
            }
        }
        self
    }

    fn redraw(&mut self) -> &mut Self {
        let path_data = self.build_path_string();
        self.set_attr("d", &path_data)
    }
}

impl Element {
    fn add_segment(&mut self, command: &str, coords: Vec<f64>) -> &mut Self {
        let segment = PathSegment {
            command: command.to_string(),
            coords,
        };

        if self.path_segments.is_none() {
            self.path_segments = Some(Vec::new());
        }

        if let Some(segments) = self.path_segments.as_mut() {
            segments.push(segment);
            
            // Auto-redraw if this is the first segment or redraw is enabled
            if segments.len() == 1 || self.auto_redraw {
                let path_data = self.build_path_string();
                self.set_attr("d", &path_data);
            }
        }
        self
    }

    fn build_path_string(&self) -> String {
        if let Some(segments) = &self.path_segments {
            let mut path_data = String::new();
            let mut last_command = "";

            for segment in segments {
                if last_command == segment.command {
                    // Same command, just add coordinates
                    path_data.push(' ');
                    path_data.push_str(&segment.coords.iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(" "));
                } else {
                    // New command
                    if !path_data.is_empty() {
                        path_data.push(' ');
                    }
                    path_data.push_str(&segment.command);
                    if !segment.coords.is_empty() {
                        path_data.push_str(&segment.coords.iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<_>>()
                            .join(" "));
                    }
                }
                last_command = &segment.command;
            }
            path_data
        } else {
            String::new()
        }
    }

    /// Enable/disable auto-redraw for path operations
    pub fn update(&mut self, auto_redraw: bool) -> &mut Self {
        self.auto_redraw = auto_redraw;
        self
    }

    /// Draw path with animation
    pub fn draw_animated(&mut self, duration: u32, delay: u32, easing: &str) -> &mut Self {
        // Calculate path length (simplified)
        let length = self.calculate_path_length();
        
        self.set_attr("stroke-dasharray", &format!("{} {}", length, length))
            .set_attr("stroke-dashoffset", &length.to_string());
        
        // Add animation (simplified - would need full animation system)
        self.animate_attr("stroke-dashoffset", &length.to_string(), "0", duration);
        
        self
    }

    fn calculate_path_length(&self) -> f64 {
        // Simplified path length calculation
        // In a real implementation, this would calculate the actual path length
        if let Some(segments) = &self.path_segments {
            segments.len() as f64 * 100.0 // Rough approximation
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg::Svg;

    #[test]
    fn test_path_building() {
        let mut canvas = Svg::new(100, 100);
        let path = canvas.path("")
            .M(10.0, 10.0)
            .L(50.0, 50.0)
            .Z();
        
        assert_eq!(path.get_segment_count(), 3);
    }

    #[test]
    fn test_path_curves() {
        let mut canvas = Svg::new(100, 100);
        let path = canvas.path("")
            .M(10.0, 10.0)
            .C(20.0, 20.0, 30.0, 30.0, 40.0, 40.0)
            .Q(50.0, 50.0, 60.0, 60.0);
        
        assert_eq!(path.get_segment_count(), 3);
    }

    #[test]
    fn test_path_manipulation() {
        let mut canvas = Svg::new(100, 100);
        let path = canvas.path("")
            .M(10.0, 10.0)
            .L(50.0, 50.0)
            .L(90.0, 10.0);
        
        assert_eq!(path.get_segment_count(), 3);
        
        path.remove_segment(1);
        assert_eq!(path.get_segment_count(), 2);
    }
}
