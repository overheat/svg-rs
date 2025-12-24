# svg-rs

[![Crates.io](https://img.shields.io/crates/v/svg-rs.svg)](https://crates.io/crates/svg-rs)
[![Documentation](https://docs.rs/svg-rs/badge.svg)](https://docs.rs/svg-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/your-username/svg-rs/workflows/CI/badge.svg)](https://github.com/your-username/svg-rs/actions)

一个轻量级的 Rust SVG 图形创建和操作库，灵感来源于 [svg.js](https://github.com/svgdotjs/svg.js)。

**[English](README.md) | [示例](examples/) | [API 文档](https://docs.rs/svg-rs)**

## ✨ 功能特性

- 🎨 **完整的 SVG 元素支持**：矩形、圆形、椭圆、线条、路径、文本、图片等
- 🔗 **流畅的 API 设计**：类似 svg.js 的链式方法调用
- 🎭 **高级图形功能**：渐变、图案、遮罩、裁剪路径
- 🎬 **动画支持**：SVG 动画与属性插值
- 📝 **文本处理**：高级文本处理，支持 tspan 和 textPath
- 🔄 **变换操作**：旋转、缩放、平移、倾斜、翻转
- 🎨 **CSS 集成**：类管理和内联样式
- 🚀 **零依赖**：纯 Rust 实现
- 🛡️ **类型安全**：利用 Rust 类型系统确保正确性
- 📦 **轻量级**：最小占用，最大功能

## 🚀 快速开始

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
svg-rs = "0.1"
```

### 基础用法

```rust
use svg_rs::*;

fn main() {
    let mut canvas = Svg::new(800, 600);

    // 创建矩形
    canvas.rect(100, 100)
        .fill("#f06")
        .move_to(50, 50);

    // 创建带动画的圆形
    let circle = canvas.circle(25)
        .fill("#0f6")
        .center(200, 100);
    circle.animate_attr("r", "25", "50", 2);

    // 保存到文件
    canvas.save("output.svg").unwrap();
    println!("SVG 已保存到 output.svg");
}
```

## 📚 示例

### 渐变和图案

```rust
let mut canvas = Svg::new(400, 300);

// 创建渐变
let defs = canvas.defs();
let gradient = defs.linear_gradient("myGradient");
gradient.stop("0%", "#ff0000");
gradient.stop("100%", "#0000ff");

// 使用渐变
canvas.rect(200, 100)
    .fill("url(#myGradient)")
    .move_to(50, 50);
```

### 高级文本处理

```rust
let text = canvas.text("")
    .move_to(50, 100)
    .font_family("Arial")
    .font_size(18);

text.tspan("你好 ").fill("#000");
text.tspan("世界！").fill("#f06").font_weight("bold");
```

### 变换和动画

```rust
// 复杂变换
canvas.rect(60, 60)
    .fill("#f0f")
    .move_to(100, 100)
    .rotate(45.0)
    .skew_x(15.0)
    .animate_attr("fill", "#f0f", "#0ff", 3);

// 分组变换
let group = canvas.group()
    .transform("translate(100, 100) scale(1.5)");

group.circle(30).fill("#red").center(0, 0);
group.rect(40, 40).fill("#blue").move_to(-20, 40);
```

### 交互元素

```rust
canvas.rect(120, 40)
    .fill("#2ecc71")
    .move_to(50, 200)
    .class("interactive")
    .on_click("this.style.fill='#27ae60'")
    .on_hover("this.style.opacity='0.8'");
```

## 📖 API 参考

### 创建元素
| 方法 | 描述 | 示例 |
|------|------|------|
| `rect(width, height)` | 矩形 | `canvas.rect(100, 50)` |
| `circle(radius)` | 圆形 | `canvas.circle(25)` |
| `ellipse(rx, ry)` | 椭圆 | `canvas.ellipse(50, 30)` |
| `line(x1, y1, x2, y2)` | 线条 | `canvas.line(0, 0, 100, 100)` |
| `path(d)` | 路径 | `canvas.path("M 10 10 L 90 90")` |
| `text(content)` | 文本 | `canvas.text("你好")` |
| `image(href, w, h)` | 图片 | `canvas.image("pic.jpg", 100, 100)` |
| `polygon(points)` | 多边形 | `canvas.polygon("0,0 50,0 25,50")` |
| `polyline(points)` | 折线 | `canvas.polyline("0,0 50,25 100,0")` |

### 样式方法
| 方法 | 描述 | 示例 |
|------|------|------|
| `fill(color)` | 填充色 | `.fill("#ff0000")` |
| `stroke(color)` | 描边色 | `.stroke("#000000")` |
| `stroke_width(width)` | 描边宽度 | `.stroke_width(2)` |
| `opacity(value)` | 透明度 | `.opacity(0.5)` |
| `class(name)` | CSS 类 | `.class("highlight")` |
| `style(css)` | 内联样式 | `.style("fill: red;")` |

### 位置和变换
| 方法 | 描述 | 示例 |
|------|------|------|
| `move_to(x, y)` | 位置 | `.move_to(50, 100)` |
| `center(x, y)` | 中心点 | `.center(100, 100)` |
| `rotate(angle)` | 旋转 | `.rotate(45.0)` |
| `scale(x, y)` | 缩放 | `.scale(2.0, 1.5)` |
| `translate(x, y)` | 平移 | `.translate(10, 20)` |
| `skew(x, y)` | 倾斜 | `.skew(15.0, 0.0)` |
| `flip(axis)` | 翻转 | `.flip("x")` |

### 高级功能
| 方法 | 描述 | 示例 |
|------|------|------|
| `animate_attr(attr, from, to, dur)` | 动画 | `.animate_attr("r", "10", "50", 2)` |
| `mask(id)` | 应用遮罩 | `.mask("myMask")` |
| `clip_path(id)` | 应用裁剪 | `.clip_path("myClip")` |
| `marker_start/mid/end(id)` | 路径标记 | `.marker_end("arrow")` |
| `viewbox(x, y, w, h)` | 设置视口 | `canvas.viewbox(0, 0, 100, 100)` |

## 🎯 与 svg.js 的对比

svg-rs 提供了约 **95%** 的 svg.js 功能，同时具备 Rust 的类型安全和性能优势：

| 功能 | svg.js | svg-rs | 状态 |
|------|--------|--------|------|
| 基础元素 | ✅ | ✅ | 完整 |
| 动画 | ✅ | ✅ | 完整 |
| 渐变/图案 | ✅ | ✅ | 完整 |
| 文本处理 | ✅ | ✅ | 完整 |
| 变换 | ✅ | ✅ | 完整 |
| CSS 集成 | ✅ | ✅ | 完整 |
| 分组 | ✅ | ✅ | 完整 |
| 遮罩/裁剪 | ✅ | ✅ | 完整 |
| 事件 | ✅ | ✅ | 基础支持 |
| DOM 操作 | ✅ | ⚠️ | 部分支持 |

## 🏃‍♂️ 运行示例

查看 [examples](examples/) 目录获取完整的使用示例：

```bash
# 基础形状和样式
cargo run --example shapes

# 高级功能演示
cargo run --example advanced

# 渐变和图案
cargo run --example gradients

# 动画
cargo run --example animations

# 交互元素
cargo run --example events
```

## 🛠️ 开发

### 前置要求

- Rust 1.70.0 或更高版本
- Cargo

### 构建

```bash
git clone https://github.com/your-username/svg-rs
cd svg-rs
cargo build
```

### 测试

```bash
cargo test
```

### 文档

```bash
cargo doc --open
```

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。对于重大更改，请先开 issue 讨论您想要更改的内容。

1. Fork 仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- 灵感来源于 [svg.js](https://github.com/svgdotjs/svg.js) - JavaScript SVG 库
- 用 ❤️ 和 Rust 构建
- 感谢所有贡献者和 Rust 社区

## 🔗 相关项目

- [svg.js](https://github.com/svgdotjs/svg.js) - 原始的 JavaScript 库
- [resvg](https://github.com/RazrFalcon/resvg) - SVG 渲染库
- [usvg](https://github.com/RazrFalcon/resvg/tree/master/crates/usvg) - SVG 解析器

---

**如果这个项目对你有帮助，请给个 ⭐ Star！**
