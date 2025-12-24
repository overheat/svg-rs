use std::collections::HashMap;

pub struct Svg {
    width: u32,
    height: u32,
    elements: Vec<Element>,
}

pub struct Element {
    tag: String,
    attributes: HashMap<String, String>,
}

impl Svg {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            elements: Vec::new(),
        }
    }

    pub fn rect(&mut self, width: u32, height: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("width".to_string(), width.to_string());
        attrs.insert("height".to_string(), height.to_string());
        
        self.add_element("rect", attrs)
    }

    pub fn circle(&mut self, radius: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("r".to_string(), radius.to_string());
        
        self.add_element("circle", attrs)
    }

    pub fn ellipse(&mut self, rx: u32, ry: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("rx".to_string(), rx.to_string());
        attrs.insert("ry".to_string(), ry.to_string());
        
        self.add_element("ellipse", attrs)
    }

    pub fn line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("x1".to_string(), x1.to_string());
        attrs.insert("y1".to_string(), y1.to_string());
        attrs.insert("x2".to_string(), x2.to_string());
        attrs.insert("y2".to_string(), y2.to_string());
        
        self.add_element("line", attrs)
    }

    pub fn path(&mut self, d: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("d".to_string(), d.to_string());
        
        self.add_element("path", attrs)
    }

    pub fn text(&mut self, content: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("text-content".to_string(), content.to_string());
        
        self.add_element("text", attrs)
    }

    fn add_element(&mut self, tag: &str, attributes: HashMap<String, String>) -> &mut Element {
        let element = Element {
            tag: tag.to_string(),
            attributes,
        };
        
        self.elements.push(element);
        self.elements.last_mut().unwrap()
    }

    pub fn to_string(&self) -> String {
        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
"#,
            self.width, self.height
        );
        
        for element in &self.elements {
            svg.push_str("  ");
            svg.push_str(&element.to_string());
            svg.push('\n');
        }
        
        svg.push_str("</svg>");
        svg
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, self.to_string())
    }
}

impl Element {
    pub fn fill(&mut self, color: &str) -> &mut Self {
        self.attributes.insert("fill".to_string(), color.to_string());
        self
    }

    pub fn stroke(&mut self, color: &str) -> &mut Self {
        self.attributes.insert("stroke".to_string(), color.to_string());
        self
    }

    pub fn stroke_width(&mut self, width: u32) -> &mut Self {
        self.attributes.insert("stroke-width".to_string(), width.to_string());
        self
    }

    pub fn opacity(&mut self, value: f32) -> &mut Self {
        self.attributes.insert("opacity".to_string(), value.to_string());
        self
    }

    pub fn move_to(&mut self, x: u32, y: u32) -> &mut Self {
        self.attributes.insert("x".to_string(), x.to_string());
        self.attributes.insert("y".to_string(), y.to_string());
        self
    }

    pub fn center(&mut self, x: u32, y: u32) -> &mut Self {
        self.attributes.insert("cx".to_string(), x.to_string());
        self.attributes.insert("cy".to_string(), y.to_string());
        self
    }

    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.attributes.insert("width".to_string(), width.to_string());
        self.attributes.insert("height".to_string(), height.to_string());
        self
    }

    pub fn transform(&mut self, transform: &str) -> &mut Self {
        self.attributes.insert("transform".to_string(), transform.to_string());
        self
    }

    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        let transform = format!("rotate({})", angle);
        self.transform(&transform)
    }

    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        let transform = format!("scale({}, {})", x, y);
        self.transform(&transform)
    }

    pub fn translate(&mut self, x: f32, y: f32) -> &mut Self {
        let transform = format!("translate({}, {})", x, y);
        self.transform(&transform)
    }

    pub fn to_string(&self) -> String {
        let mut attrs = String::new();
        let mut text_content = String::new();
        
        for (key, value) in &self.attributes {
            if key == "text-content" {
                text_content = value.clone();
            } else {
                attrs.push_str(&format!(r#" {}="{}""#, key, value));
            }
        }
        
        if self.tag == "text" && !text_content.is_empty() {
            format!("<{}{}>{}</{}>", self.tag, attrs, text_content, self.tag)
        } else {
            format!("<{}{}/>", self.tag, attrs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_creation() {
        let svg = Svg::new(100, 200);
        let output = svg.to_string();
        assert!(output.contains("width=\"100\""));
        assert!(output.contains("height=\"200\""));
        assert!(output.contains("xmlns=\"http://www.w3.org/2000/svg\""));
    }

    #[test]
    fn test_rect_creation() {
        let mut svg = Svg::new(100, 100);
        svg.rect(50, 30)
            .fill("#ff0000")
            .move_to(10, 20);
        
        let output = svg.to_string();
        assert!(output.contains("rect"));
        assert!(output.contains("width=\"50\""));
        assert!(output.contains("height=\"30\""));
        assert!(output.contains("fill=\"#ff0000\""));
        assert!(output.contains("x=\"10\""));
        assert!(output.contains("y=\"20\""));
    }

    #[test]
    fn test_circle_creation() {
        let mut svg = Svg::new(100, 100);
        svg.circle(25)
            .fill("#00ff00")
            .center(50, 60);
        
        let output = svg.to_string();
        assert!(output.contains("circle"));
        assert!(output.contains("r=\"25\""));
        assert!(output.contains("fill=\"#00ff00\""));
        assert!(output.contains("cx=\"50\""));
        assert!(output.contains("cy=\"60\""));
    }

    #[test]
    fn test_transforms() {
        let mut svg = Svg::new(100, 100);
        svg.rect(20, 20)
            .move_to(10, 10)
            .rotate(45.0);
        
        let output = svg.to_string();
        assert!(output.contains("transform=\"rotate(45)\""));
    }
}
