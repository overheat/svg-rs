# SVG-RS: Rust 版本的 svg.js

这是一个用 Rust 语言重写的 svg.js 库的核心功能实现。

## 🎯 项目目标

将流行的 JavaScript SVG 操作库 [svg.js](https://github.com/svgdotjs/svg.js) 的核心功能移植到 Rust，提供：

- 类似的链式 API 设计
- 轻量级的 SVG 创建和操作
- 类型安全的 Rust 实现
- 零依赖的纯 Rust 解决方案

## ✨ 已实现功能

### 基础元素创建
- ✅ `rect(width, height)` - 矩形
- ✅ `circle(radius)` - 圆形  
- ✅ `ellipse(rx, ry)` - 椭圆
- ✅ `line(x1, y1, x2, y2)` - 线条
- ✅ `path(d)` - 路径
- ✅ `text(content)` - 文本

### 样式属性
- ✅ `fill(color)` - 填充色
- ✅ `stroke(color)` - 描边色
- ✅ `stroke_width(width)` - 描边宽度
- ✅ `opacity(value)` - 透明度

### 位置和变换
- ✅ `move_to(x, y)` - 移动位置
- ✅ `center(x, y)` - 设置中心点
- ✅ `rotate(angle)` - 旋转
- ✅ `scale(x, y)` - 缩放
- ✅ `translate(x, y)` - 平移
- ✅ `transform(transform)` - 自定义变换

### 输出功能
- ✅ `to_string()` - 生成 SVG 字符串
- ✅ `save(filename)` - 保存到文件

## 🚀 使用示例

```rust
use svg_rs::*;

let mut canvas = Svg::new(800, 600);

// 创建矩形
canvas.rect(100, 100)
    .fill("#f06")
    .move_to(50, 50);

// 创建圆形
canvas.circle(50)
    .fill("#0f6")
    .center(200, 100);

// 创建带变换的元素
canvas.rect(60, 60)
    .fill("#f0f")
    .move_to(400, 250)
    .rotate(45.0);

// 保存到文件
canvas.save("output.svg").unwrap();
```

## 📁 项目结构

```
svg-rs/
├── src/
│   ├── lib.rs          # 库入口
│   ├── svg.rs          # 核心实现
│   └── main.rs         # 示例程序
├── examples/
│   ├── quickstart.rs   # 快速上手示例
│   ├── advanced.rs     # 进阶特性示例
│   └── feature_tests.rs # 手动特性测试
├── Cargo.toml          # 项目配置
└── README.md           # 文档
```

## 🧪 测试

项目包含完整的单元测试：

```bash
cargo test
```

测试覆盖：
- SVG 画布创建
- 各种元素创建
- 样式属性设置
- 变换操作

## 🎨 生成的 SVG 示例

运行 `cargo run --example quickstart` 会生成一个包含文本、圆形、矩形描边和基准线的简单 SVG；`cargo run --example advanced` 演示渐变、分组变换、路径、动画和交互按钮。

## 🔄 与 svg.js 的对比

| 功能 | svg.js | svg-rs | 状态 |
|------|--------|--------|------|
| 基础元素 | ✅ | ✅ | 完成 |
| 链式 API | ✅ | ✅ | 完成 |
| 样式属性 | ✅ | ✅ | 完成 |
| 变换操作 | ✅ | ✅ | 完成 |
| 动画 | ✅ | ❌ | 待实现 |
| 事件处理 | ✅ | ❌ | 待实现 |
| 渐变/图案 | ✅ | ❌ | 待实现 |
| 分组 | ✅ | ❌ | 待实现 |

## 🛠 技术特点

- **零依赖**: 纯 Rust 实现，无外部依赖
- **类型安全**: 利用 Rust 的类型系统防止运行时错误
- **内存安全**: 无需担心内存泄漏或悬空指针
- **性能优化**: Rust 的零成本抽象和编译时优化
- **链式 API**: 保持与 svg.js 相似的使用体验

## 📈 未来计划

1. **动画支持** - 实现基础的 SVG 动画功能
2. **渐变和图案** - 支持线性/径向渐变和填充图案
3. **分组功能** - 实现 `<g>` 元素支持
4. **事件系统** - 添加交互事件处理
5. **更多元素** - 支持更多 SVG 元素类型
6. **优化输出** - 更好的 SVG 代码格式化

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来完善这个项目！

## 📄 许可证

MIT License - 与原始 svg.js 保持一致
