/// Draggable functionality for SVG elements
#[cfg(feature = "draggable")]
pub struct DragHandler {
    pub enabled: bool,
    pub constraints: Option<(f32, f32, f32, f32)>, // x, y, width, height
    pub grid_snap: Option<f32>,
}

#[cfg(feature = "draggable")]
impl Default for DragHandler {
    fn default() -> Self {
        Self {
            enabled: false,
            constraints: None,
            grid_snap: None,
        }
    }
}

#[cfg(feature = "draggable")]
impl DragHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(&mut self) -> &mut Self {
        self.enabled = true;
        self
    }

    pub fn disable(&mut self) -> &mut Self {
        self.enabled = false;
        self
    }

    pub fn constrain(&mut self, x: f32, y: f32, width: f32, height: f32) -> &mut Self {
        self.constraints = Some((x, y, width, height));
        self
    }

    pub fn snap_to_grid(&mut self, size: f32) -> &mut Self {
        self.grid_snap = Some(size);
        self
    }

    pub fn generate_script(&self, element_id: &str) -> String {
        if !self.enabled {
            return String::new();
        }

        let mut script = format!(r#"
<script>
(function() {{
    const element = document.getElementById('{}');
    if (!element) return;
    
    let isDragging = false;
    let startX, startY, initialX, initialY;
    
    function getMousePos(e) {{
        const rect = element.ownerSVGElement.getBoundingClientRect();
        return {{
            x: e.clientX - rect.left,
            y: e.clientY - rect.top
        }};
    }}
    
    function onMouseDown(e) {{
        e.preventDefault();
        const pos = getMousePos(e);
        startX = pos.x;
        startY = pos.y;
        
        const transform = element.getAttribute('transform') || '';
        const match = transform.match(/translate\(([^,]+),\s*([^)]+)\)/);
        initialX = match ? parseFloat(match[1]) : 0;
        initialY = match ? parseFloat(match[2]) : 0;
        
        isDragging = true;
        element.dispatchEvent(new CustomEvent('dragstart', {{ detail: {{ x: initialX, y: initialY }} }}));
        
        document.addEventListener('mousemove', onMouseMove);
        document.addEventListener('mouseup', onMouseUp);
    }}
    
    function onMouseMove(e) {{
        if (!isDragging) return;
        
        const pos = getMousePos(e);
        let newX = initialX + (pos.x - startX);
        let newY = initialY + (pos.y - startY);
"#, element_id);

        // Add constraints
        if let Some((cx, cy, cw, ch)) = self.constraints {
            script.push_str(&format!(r#"
        // Apply constraints
        newX = Math.max({}, Math.min({}, newX));
        newY = Math.max({}, Math.min({}, newY));
"#, cx, cx + cw, cy, cy + ch));
        }

        // Add grid snapping
        if let Some(grid_size) = self.grid_snap {
            script.push_str(&format!(r#"
        // Snap to grid
        newX = Math.round(newX / {}) * {};
        newY = Math.round(newY / {}) * {};
"#, grid_size, grid_size, grid_size, grid_size));
        }

        script.push_str(r#"
        element.setAttribute('transform', `translate(${newX}, ${newY})`);
        element.dispatchEvent(new CustomEvent('dragmove', { detail: { x: newX, y: newY } }));
    }
    
    function onMouseUp(e) {
        if (!isDragging) return;
        
        isDragging = false;
        const transform = element.getAttribute('transform') || '';
        const match = transform.match(/translate\(([^,]+),\s*([^)]+)\)/);
        const finalX = match ? parseFloat(match[1]) : 0;
        const finalY = match ? parseFloat(match[2]) : 0;
        
        element.dispatchEvent(new CustomEvent('dragend', { detail: { x: finalX, y: finalY } }));
        
        document.removeEventListener('mousemove', onMouseMove);
        document.removeEventListener('mouseup', onMouseUp);
    }
    
    element.addEventListener('mousedown', onMouseDown);
    element.style.cursor = 'move';
})();
</script>
"#);

        script
    }
}
