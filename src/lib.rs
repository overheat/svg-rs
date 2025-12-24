//! # svg-rs
//!
//! A lightweight Rust library for creating and manipulating SVG graphics, inspired by svg.js.
//!
//! ## Features
//!
//! - Complete SVG element support: rectangles, circles, ellipses, lines, paths, text, images, and more
//! - Fluent API design: chainable methods similar to svg.js
//! - Advanced graphics: gradients, patterns, masks, clipping paths
//! - Animations: SVG animations with attribute interpolation
//! - Text processing: advanced text handling with tspan and textPath
//! - Transformations: rotate, scale, translate, skew, flip
//! - CSS integration: class management and inline styles
//! - Zero dependencies: pure Rust implementation
//! - Type safety: leverages Rust's type system for correctness
//!
//! ## Quick Start
//!
//! ```rust
//! use svg_rs::*;
//!
//! let mut canvas = Svg::new(800, 600);
//!
//! // Create a rectangle
//! canvas.rect(100, 100)
//!     .fill("#f06")
//!     .move_to(50, 50);
//!
//! // Create a circle
//! canvas.circle(50)
//!     .fill("#0f6")
//!     .center(200, 100);
//!
//! // Save to file
//! canvas.save("output.svg").unwrap();
//! ```

pub mod svg;

#[cfg(feature = "draggable")]
pub mod draggable;

#[cfg(feature = "shapes")]
pub mod shapes;

#[cfg(feature = "math")]
pub mod math;

#[cfg(feature = "path")]
pub mod path;

pub use svg::*;

#[cfg(feature = "draggable")]
pub use draggable::*;

#[cfg(feature = "shapes")]
pub use shapes::*;

#[cfg(feature = "math")]
pub use math::*;

#[cfg(feature = "path")]
pub use path::*;
