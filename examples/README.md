# SVG-RS Examples

This directory provides feature-specific examples for easy reference and demonstration.

Run from project root directory:

```bash
cargo run --example <example_name>
```

Example list (`<example_name>` corresponds to filename):

- `quickstart` — Minimal runnable example showing fill, stroke, text and lines.
- `shapes` — Basic geometry: rectangles, circles, polygons, polylines.
- `path` — Using `path` to draw custom curved paths.
- `grouping` — Grouping and transform combinations (`<g>` + translate/rotate).
- `gradients` — Define and apply linear gradients.
- `clip` — Using `clipPath` to clip elements.
- `mask` — Using `mask` to control visibility.
- `styles` — Inline `<style>` mixed with class/inline styles.
- `animations` — Attribute animations based on `animate`.
- `events` — Bind `on_click` / `on_hover` events (requires browser for interaction).
- `advanced` — Comprehensive example: gradients, grouping, paths, animations and interactive buttons.
