# svg-rs
The lightweight library for manipulating and animating SVG in rust.

## 功能特性

- 创建和操作 SVG 元素（矩形、圆形、椭圆、线条、路径、文本）
- 链式 API 设计，类似 svg.js
- 支持填充、描边、变换等属性
- 保存 SVG 到文件

## 使用示例

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

## API 参考

### 创建元素
- `rect(width, height)` - 创建矩形
- `circle(radius)` - 创建圆形
- `ellipse(rx, ry)` - 创建椭圆
- `line(x1, y1, x2, y2)` - 创建线条
- `path(d)` - 创建路径
- `text(content)` - 创建文本

### 样式方法
- `fill(color)` - 设置填充色
- `stroke(color)` - 设置描边色
- `stroke_width(width)` - 设置描边宽度
- `opacity(value)` - 设置透明度

### 位置和变换
- `move_to(x, y)` - 移动到指定位置
- `center(x, y)` - 设置中心点
- `rotate(angle)` - 旋转
- `scale(x, y)` - 缩放
- `translate(x, y)` - 平移
