use std::collections::HashMap;

pub struct Svg {
    width: u32,
    height: u32,
    viewbox: Option<(f32, f32, f32, f32)>,
    elements: Vec<Element>,
}

pub struct Element {
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<Element>,
}

pub struct Group<'a> {
    element: &'a mut Element,
    svg: &'a mut Svg,
}

impl Svg {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            viewbox: None,
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

    pub fn group(&mut self) -> &mut Element {
        self.add_element("g", HashMap::new())
    }

    pub fn polyline(&mut self, points: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("points".to_string(), points.to_string());
        self.add_element("polyline", attrs)
    }

    pub fn polygon(&mut self, points: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("points".to_string(), points.to_string());
        self.add_element("polygon", attrs)
    }

    pub fn defs(&mut self) -> &mut Element {
        self.add_element("defs", HashMap::new())
    }

    pub fn mask(&mut self, id: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), id.to_string());
        self.add_element("mask", attrs)
    }

    pub fn clip_path(&mut self, id: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), id.to_string());
        self.add_element("clipPath", attrs)
    }

    pub fn style_element(&mut self, css: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("text-content".to_string(), css.to_string());
        self.add_element("style", attrs)
    }

    pub fn image(&mut self, href: &str, width: u32, height: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("href".to_string(), href.to_string());
        attrs.insert("width".to_string(), width.to_string());
        attrs.insert("height".to_string(), height.to_string());
        self.add_element("image", attrs)
    }

    pub fn use_element(&mut self, href: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("href".to_string(), format!("#{}", href));
        self.add_element("use", attrs)
    }

    pub fn marker(&mut self, id: &str, width: u32, height: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), id.to_string());
        attrs.insert("markerWidth".to_string(), width.to_string());
        attrs.insert("markerHeight".to_string(), height.to_string());
        attrs.insert("refX".to_string(), "0".to_string());
        attrs.insert("refY".to_string(), "0".to_string());
        attrs.insert("orient".to_string(), "auto".to_string());
        self.add_element("marker", attrs)
    }

    pub fn pattern(&mut self, id: &str, width: u32, height: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), id.to_string());
        attrs.insert("width".to_string(), width.to_string());
        attrs.insert("height".to_string(), height.to_string());
        attrs.insert("patternUnits".to_string(), "userSpaceOnUse".to_string());
        self.add_element("pattern", attrs)
    }

    pub fn symbol(&mut self, id: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), id.to_string());
        self.add_element("symbol", attrs)
    }

    pub fn foreign_object(&mut self, width: u32, height: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("width".to_string(), width.to_string());
        attrs.insert("height".to_string(), height.to_string());
        self.add_element("foreignObject", attrs)
    }

    pub fn link(&mut self, href: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("href".to_string(), href.to_string());
        self.add_element("a", attrs)
    }

    fn add_element(&mut self, tag: &str, attributes: HashMap<String, String>) -> &mut Element {
        let element = Element {
            tag: tag.to_string(),
            attributes,
            children: Vec::new(),
        };
        
        self.elements.push(element);
        self.elements.last_mut().unwrap()
    }

    pub fn to_string(&self) -> String {
        let mut svg = format!(
            r#"<svg width="{}" height="{}""#,
            self.width, self.height
        );
        
        if let Some((x, y, w, h)) = self.viewbox {
            svg.push_str(&format!(r#" viewBox="{} {} {} {}""#, x, y, w, h));
        }
        
        svg.push_str(r#" xmlns="http://www.w3.org/2000/svg">
"#);
        
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

    pub fn viewbox(&mut self, x: f32, y: f32, width: f32, height: f32) -> &mut Self {
        self.viewbox = Some((x, y, width, height));
        self
    }
}

impl Element {
    pub fn add_child(&mut self, tag: &str, attributes: HashMap<String, String>) -> &mut Element {
        let element = Element {
            tag: tag.to_string(),
            attributes,
            children: Vec::new(),
        };
        self.children.push(element);
        self.children.last_mut().unwrap()
    }

    pub fn rect(&mut self, width: u32, height: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("width".to_string(), width.to_string());
        attrs.insert("height".to_string(), height.to_string());
        self.add_child("rect", attrs)
    }

    pub fn circle(&mut self, radius: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("r".to_string(), radius.to_string());
        self.add_child("circle", attrs)
    }

    pub fn linear_gradient(&mut self, id: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), id.to_string());
        self.add_child("linearGradient", attrs)
    }

    pub fn stop(&mut self, offset: &str, color: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("offset".to_string(), offset.to_string());
        attrs.insert("stop-color".to_string(), color.to_string());
        self.add_child("stop", attrs)
    }

    pub fn ellipse(&mut self, rx: u32, ry: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("rx".to_string(), rx.to_string());
        attrs.insert("ry".to_string(), ry.to_string());
        self.add_child("ellipse", attrs)
    }

    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("x1".to_string(), x1.to_string());
        attrs.insert("y1".to_string(), y1.to_string());
        attrs.insert("x2".to_string(), x2.to_string());
        attrs.insert("y2".to_string(), y2.to_string());
        self.add_child("line", attrs)
    }

    pub fn path(&mut self, d: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("d".to_string(), d.to_string());
        self.add_child("path", attrs)
    }

    pub fn polygon(&mut self, points: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("points".to_string(), points.to_string());
        self.add_child("polygon", attrs)
    }

    pub fn polyline(&mut self, points: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("points".to_string(), points.to_string());
        self.add_child("polyline", attrs)
    }

    pub fn text(&mut self, content: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("text-content".to_string(), content.to_string());
        self.add_child("text", attrs)
    }

    pub fn group(&mut self) -> &mut Element {
        self.add_child("g", HashMap::new())
    }

    pub fn tspan(&mut self, content: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("text-content".to_string(), content.to_string());
        self.add_child("tspan", attrs)
    }

    pub fn text_path(&mut self, path_id: &str, content: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("href".to_string(), format!("#{}", path_id));
        attrs.insert("text-content".to_string(), content.to_string());
        self.add_child("textPath", attrs)
    }

    pub fn image(&mut self, href: &str, width: u32, height: u32) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("href".to_string(), href.to_string());
        attrs.insert("width".to_string(), width.to_string());
        attrs.insert("height".to_string(), height.to_string());
        self.add_child("image", attrs)
    }

    pub fn use_element(&mut self, href: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("href".to_string(), format!("#{}", href));
        self.add_child("use", attrs)
    }

    pub fn symbol(&mut self, id: &str) -> &mut Element {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), id.to_string());
        self.add_child("symbol", attrs)
    }

    pub fn animate(&mut self, duration: u32) -> &mut Self {
        let mut attrs = HashMap::new();
        attrs.insert("dur".to_string(), format!("{}s", duration));
        let animate_elem = Element {
            tag: "animate".to_string(),
            attributes: attrs,
            children: Vec::new(),
        };
        self.children.push(animate_elem);
        self
    }

    pub fn animate_attr(&mut self, attr: &str, from: &str, to: &str, duration: u32) -> &mut Self {
        let mut attrs = HashMap::new();
        attrs.insert("attributeName".to_string(), attr.to_string());
        attrs.insert("from".to_string(), from.to_string());
        attrs.insert("to".to_string(), to.to_string());
        attrs.insert("dur".to_string(), format!("{}s", duration));
        attrs.insert("repeatCount".to_string(), "indefinite".to_string());
        
        let animate_elem = Element {
            tag: "animate".to_string(),
            attributes: attrs,
            children: Vec::new(),
        };
        self.children.push(animate_elem);
        self
    }

    pub fn on_click(&mut self, handler: &str) -> &mut Self {
        self.attributes.insert("onclick".to_string(), handler.to_string());
        self
    }

    pub fn on_hover(&mut self, handler: &str) -> &mut Self {
        self.attributes.insert("onmouseover".to_string(), handler.to_string());
        self
    }

    pub fn class(&mut self, class_name: &str) -> &mut Self {
        self.attributes.insert("class".to_string(), class_name.to_string());
        self
    }

    pub fn add_class(&mut self, class_name: &str) -> &mut Self {
        let current = self.attributes.get("class").cloned().unwrap_or_default();
        let new_class = if current.is_empty() {
            class_name.to_string()
        } else {
            format!("{} {}", current, class_name)
        };
        self.attributes.insert("class".to_string(), new_class);
        self
    }

    pub fn style(&mut self, style: &str) -> &mut Self {
        self.attributes.insert("style".to_string(), style.to_string());
        self
    }

    pub fn mask(&mut self, mask_id: &str) -> &mut Self {
        self.attributes.insert("mask".to_string(), format!("url(#{})", mask_id));
        self
    }

    pub fn clip_path(&mut self, clip_id: &str) -> &mut Self {
        self.attributes.insert("clip-path".to_string(), format!("url(#{})", clip_id));
        self
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.attributes.insert("id".to_string(), id.to_string());
        self
    }

    pub fn font_family(&mut self, family: &str) -> &mut Self {
        self.attributes.insert("font-family".to_string(), family.to_string());
        self
    }

    pub fn font_size(&mut self, size: u32) -> &mut Self {
        self.attributes.insert("font-size".to_string(), size.to_string());
        self
    }

    pub fn font_weight(&mut self, weight: &str) -> &mut Self {
        self.attributes.insert("font-weight".to_string(), weight.to_string());
        self
    }

    pub fn font_style(&mut self, style: &str) -> &mut Self {
        self.attributes.insert("font-style".to_string(), style.to_string());
        self
    }

    pub fn text_anchor(&mut self, anchor: &str) -> &mut Self {
        self.attributes.insert("text-anchor".to_string(), anchor.to_string());
        self
    }

    pub fn dx(&mut self, value: f32) -> &mut Self {
        self.attributes.insert("dx".to_string(), value.to_string());
        self
    }

    pub fn dy(&mut self, value: f32) -> &mut Self {
        self.attributes.insert("dy".to_string(), value.to_string());
        self
    }

    pub fn marker_start(&mut self, marker_id: &str) -> &mut Self {
        self.attributes.insert("marker-start".to_string(), format!("url(#{})", marker_id));
        self
    }

    pub fn marker_mid(&mut self, marker_id: &str) -> &mut Self {
        self.attributes.insert("marker-mid".to_string(), format!("url(#{})", marker_id));
        self
    }

    pub fn marker_end(&mut self, marker_id: &str) -> &mut Self {
        self.attributes.insert("marker-end".to_string(), format!("url(#{})", marker_id));
        self
    }

    pub fn attr(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }

    pub fn set_attr(&mut self, name: &str, value: &str) -> &mut Self {
        self.attributes.insert(name.to_string(), value.to_string());
        self
    }

    pub fn remove_class(&mut self, class_name: &str) -> &mut Self {
        if let Some(current) = self.attributes.get("class") {
            let classes: Vec<&str> = current.split_whitespace()
                .filter(|&c| c != class_name)
                .collect();
            if classes.is_empty() {
                self.attributes.remove("class");
            } else {
                self.attributes.insert("class".to_string(), classes.join(" "));
            }
        }
        self
    }

    pub fn toggle_class(&mut self, class_name: &str) -> &mut Self {
        if self.has_class(class_name) {
            self.remove_class(class_name)
        } else {
            self.add_class(class_name)
        }
    }

    pub fn has_class(&self, class_name: &str) -> bool {
        self.attributes.get("class")
            .map(|classes| classes.split_whitespace().any(|c| c == class_name))
            .unwrap_or(false)
    }

    pub fn hide(&mut self) -> &mut Self {
        self.style("display: none")
    }

    pub fn show(&mut self) -> &mut Self {
        let current_style = self.attributes.get("style").cloned().unwrap_or_default();
        let new_style = current_style.replace("display: none", "").trim().to_string();
        if new_style.is_empty() {
            self.attributes.remove("style");
        } else {
            self.attributes.insert("style".to_string(), new_style);
        }
        self
    }

    pub fn visible(&self) -> bool {
        !self.attributes.get("style")
            .map(|s| s.contains("display: none"))
            .unwrap_or(false)
    }
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

    pub fn move_to(&mut self, x: i32, y: i32) -> &mut Self {
        self.attributes.insert("x".to_string(), x.to_string());
        self.attributes.insert("y".to_string(), y.to_string());
        self
    }

    pub fn center(&mut self, x: i32, y: i32) -> &mut Self {
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

    pub fn flip(&mut self, axis: &str) -> &mut Self {
        let transform = match axis {
            "x" => "scale(-1, 1)".to_string(),
            "y" => "scale(1, -1)".to_string(),
            "both" => "scale(-1, -1)".to_string(),
            _ => return self,
        };
        self.transform(&transform)
    }

    pub fn skew(&mut self, x: f32, y: f32) -> &mut Self {
        let transform = format!("skewX({}) skewY({})", x, y);
        self.transform(&transform)
    }

    pub fn skew_x(&mut self, angle: f32) -> &mut Self {
        let transform = format!("skewX({})", angle);
        self.transform(&transform)
    }

    pub fn skew_y(&mut self, angle: f32) -> &mut Self {
        let transform = format!("skewY({})", angle);
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
        
        if !self.children.is_empty() || (self.tag == "text" && !text_content.is_empty()) {
            let mut content = text_content;
            for child in &self.children {
                content.push_str(&child.to_string());
            }
            format!("<{}{}>{}</{}>", self.tag, attrs, content, self.tag)
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

    #[test]
    fn test_group() {
        let mut svg = Svg::new(200, 200);
        let group = svg.group();
        group.rect(50, 50).fill("#red").move_to(0, 0);
        
        let output = svg.to_string();
        assert!(output.contains("<g>"));
        assert!(output.contains("</g>"));
        assert!(output.contains("rect"));
    }

    #[test]
    fn test_gradient() {
        let mut svg = Svg::new(200, 200);
        let defs = svg.defs();
        let gradient = defs.linear_gradient("test");
        gradient.stop("0%", "#ff0000");
        
        let output = svg.to_string();
        assert!(output.contains("defs"));
        assert!(output.contains("linearGradient"));
        assert!(output.contains("stop"));
    }

    #[test]
    fn test_animation() {
        let mut svg = Svg::new(100, 100);
        svg.circle(25)
            .center(50, 50)
            .animate_attr("r", "25", "50", 2);
        
        let output = svg.to_string();
        assert!(output.contains("animate"));
        assert!(output.contains("attributeName=\"r\""));
    }

    #[test]
    fn test_css_classes() {
        let mut svg = Svg::new(100, 100);
        svg.rect(50, 50)
            .class("primary")
            .add_class("highlight")
            .move_to(10, 10);
        
        let output = svg.to_string();
        assert!(output.contains("class=\"primary highlight\""));
    }

    #[test]
    fn test_mask_and_clip() {
        let mut svg = Svg::new(200, 200);
        let defs = svg.defs();
        defs.add_child("mask", {
            let mut attrs = std::collections::HashMap::new();
            attrs.insert("id".to_string(), "testMask".to_string());
            attrs
        });
        
        svg.rect(100, 100)
            .mask("testMask")
            .clip_path("testClip");
        
        let output = svg.to_string();
        assert!(output.contains("mask=\"url(#testMask)\""));
        assert!(output.contains("clip-path=\"url(#testClip)\""));
    }

    #[test]
    fn test_polyline_polygon() {
        let mut svg = Svg::new(200, 200);
        svg.polyline("10,10 50,50 100,10").stroke("#red");
        svg.polygon("10,100 50,150 100,100").fill("#blue");
        
        let output = svg.to_string();
        assert!(output.contains("polyline"));
        assert!(output.contains("polygon"));
        assert!(output.contains("points="));
    }

    #[test]
    fn test_missing_elements() {
        let mut svg = Svg::new(200, 200);
        svg.image("test.jpg", 100, 100).move_to(0, 0);
        svg.use_element("test").move_to(50, 50);
        svg.symbol("test-symbol");
        
        let output = svg.to_string();
        assert!(output.contains("image"));
        assert!(output.contains("use"));
        assert!(output.contains("symbol"));
    }

    #[test]
    fn test_text_features() {
        let mut svg = Svg::new(200, 200);
        let text = svg.text("").font_family("Arial").font_size(16);
        text.tspan("Hello").fill("#red");
        text.tspan(" World").fill("#blue");
        
        let output = svg.to_string();
        assert!(output.contains("font-family=\"Arial\""));
        assert!(output.contains("font-size=\"16\""));
        assert!(output.contains("tspan"));
    }

    #[test]
    fn test_advanced_transforms() {
        let mut svg = Svg::new(200, 200);
        svg.rect(50, 50)
            .flip("x")
            .skew(10.0, 5.0)
            .move_to(10, 10);
        
        let output = svg.to_string();
        assert!(output.contains("transform="));
        assert!(output.contains("scale(-1, 1)") || output.contains("skew"));
    }

    #[test]
    fn test_css_management() {
        let mut svg = Svg::new(200, 200);
        let rect = svg.rect(50, 50)
            .class("primary")
            .add_class("highlight")
            .move_to(10, 10);
        
        assert!(rect.has_class("primary"));
        assert!(rect.has_class("highlight"));
        
        rect.remove_class("primary");
        assert!(!rect.has_class("primary"));
        assert!(rect.has_class("highlight"));
    }

    #[test]
    fn test_viewbox() {
        let mut svg = Svg::new(800, 600);
        svg.viewbox(0.0, 0.0, 400.0, 300.0);
        
        let output = svg.to_string();
        assert!(output.contains("viewBox=\"0 0 400 300\""));
    }
}
