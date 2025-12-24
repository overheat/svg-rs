# SVG-RS Examples

此目录包含为 `svg-rs` 库准备的精简示例，每个示例仅展示单一功能以方便测试和演示。

运行方式（在项目根目录）：

```bash
cargo run --example <example_name>
```

可用示例（文件名对应 `<example_name>`）：

- `basic_shapes` — 基本形状：`rect`, `circle`, `polygon`。
- `grouping` — 分组与变换（`<g>` + transform）。
- `gradients` — 定义并应用线性渐变（`<linearGradient>`）。
- `animations` — 简单属性动画示例（`animate`/attribute changes）。
- `events` — 事件处理示例（`on_click`, `on_hover`，在浏览器查看交互）。
- `path` — 使用 `path` 绘制自定义曲线路径。
- `mask` — 演示 `mask` 的定义与应用。
- `clip` — 演示 `clipPath` 的定义与应用。
- `styles` — 插入 `<style>` 并使用 class/inline 样式。
- `polygon_polyline` — `polygon` 与 `polyline` 的用法示例。

另外，原始的综合示例已被移动到 `examples/combined/`，保留为完整演示参考（文件名以 `_combined` 结尾）。

如果你想要我运行一遍所有示例以做快速语法检查（逐个 `cargo run --example`），或更新 `examples/feature_tests.rs` 以自动运行它们，请告诉我。 
