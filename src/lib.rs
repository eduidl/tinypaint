//! TinyPaint - A lightweight painting application built with Rust and WGPU
//!
//! This library provides a simple and efficient way to create 2D painting applications
//! with hardware-accelerated rendering using WGPU.
//!
//! # Examples
//!
//! ```no_run
//! use tinypaint::draw;
//!
//! #[tokio::main]
//! async fn main() {
//!     draw(|ctx| {
//!         // Your drawing code here
//!     })
//!     .await
//!     .unwrap();
//! }
//! ```

mod canvas;
mod color;
mod command;
mod context;
mod error;
mod renderer;
mod vertex;

pub use canvas::Canvas;
pub use color::Color;
pub use context::CanvasContext;
pub use error::{Result, TinyPaintError};
pub use vertex::Point;

/// Creates a new canvas and runs the provided drawing function
///
/// # Arguments
///
/// * `width` - The width of the canvas
/// * `height` - The height of the canvas
/// * `func` - A function that takes a `CanvasContext` and performs drawing operations
///
/// # Returns
///
/// A `Result` that resolves when the canvas window is closed
///
/// # Examples
///
/// ```no_run
/// use tinypaint::{draw, Result};
///
/// async fn run() -> Result<()> {
///     draw(800, 600, |ctx| {
///         // Your drawing code here
///     })
///     .await
/// }
/// ```
pub async fn draw(
    width: u32,
    height: u32,
    func: impl Fn(CanvasContext) + Send + 'static,
) -> Result<()> {
    let canvas = Canvas::new(width, height).await?;
    let context = canvas.context();

    std::thread::spawn(move || func(context));

    canvas.run()
}
