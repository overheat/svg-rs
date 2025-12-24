# SVG-RS Examples

此目录提供分功能的示例，方便按需查阅和演示。

运行方式（项目根目录）：

```bash
cargo run --example <example_name>
```

示例列表（`<example_name>` 对应文件名）：

- `quickstart` — 最小可运行示例，展示填充、描边、文本与直线。
- `shapes` — 基础几何：矩形、圆、多边形、多段线。
- `path` — 使用 `path` 绘制自定义曲线路径。
- `grouping` — 分组与变换组合（`<g>` + translate/rotate）。
- `gradients` — 定义并应用线性渐变。
- `clip` — 使用 `clipPath` 裁剪元素。
- `mask` — 使用 `mask` 控制可见性。
- `styles` — 内联 `<style>` 与 class/inline 样式混合。
- `animations` — 基于 `animate` 的属性动画。
- `events` — 绑定 `on_click` / `on_hover` 事件（需浏览器查看交互）。
- `advanced` — 综合示例：渐变、分组、路径、动画与交互按钮。
