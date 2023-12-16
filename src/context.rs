//! Canvas context module provides the drawing interface for the painting application.
//!
//! This module defines the `CanvasContext` struct and its methods for drawing shapes
//! and managing the canvas state.

use winit::event_loop::EventLoopProxy;

use crate::{
    vertex::{DrawEvent, Vertex},
    Color,
};

/// Converts x-coordinate from pixel space to normalized device coordinates.
fn convert_x(x: u32, max: u32) -> f32 {
    (x * 2) as f32 / max as f32 - 1.0
}

/// Converts y-coordinate from pixel space to normalized device coordinates.
fn convert_y(y: u32, max: u32) -> f32 {
    1.0 - (y * 2) as f32 / max as f32
}

/// Provides the drawing interface for the canvas.
///
/// The `CanvasContext` struct manages the canvas dimensions and provides methods
/// for drawing shapes and managing the canvas state.
#[derive(Debug)]
pub struct CanvasContext {
    width: u32,
    height: u32,
    proxy: EventLoopProxy<DrawEvent>,
}

impl CanvasContext {
    /// Creates a new canvas context with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the canvas in pixels
    /// * `height` - The height of the canvas in pixels
    /// * `proxy` - The event loop proxy for sending drawing events
    pub(crate) fn new(width: u32, height: u32, proxy: EventLoopProxy<DrawEvent>) -> Self {
        Self {
            width,
            height,
            proxy,
        }
    }

    /// Returns the width of the canvas in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the canvas in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Converts a point from pixel coordinates to normalized device coordinates.
    fn convert_point(&self, p: (u32, u32), color: Color) -> Vertex {
        Vertex {
            position: [convert_x(p.0, self.width), convert_y(p.1, self.height)],
            color,
        }
    }

    /// Draws a point at the specified coordinates with the given color.
    ///
    /// # Arguments
    ///
    /// * `p0` - The coordinates of the point (x, y)
    /// * `color` - The color of the point
    pub fn draw_point(&self, p0: (u32, u32), color: Color) {
        let p0 = self.convert_point(p0, color);

        self.proxy
            .send_event(DrawEvent::Point(p0))
            .expect("Failed to send draw event");
    }

    /// Draws a line between two points with the given color.
    ///
    /// # Arguments
    ///
    /// * `p0` - The starting point coordinates (x, y)
    /// * `p1` - The ending point coordinates (x, y)
    /// * `color` - The color of the line
    pub fn draw_line(&self, p0: (u32, u32), p1: (u32, u32), color: Color) {
        let p0 = self.convert_point(p0, color);
        let p1 = self.convert_point(p1, color);

        self.proxy
            .send_event(DrawEvent::Line(p0, p1))
            .expect("Failed to send draw event");
    }

    /// Draws a triangle defined by three points with the given color.
    ///
    /// # Arguments
    ///
    /// * `p0` - The first point coordinates (x, y)
    /// * `p1` - The second point coordinates (x, y)
    /// * `p2` - The third point coordinates (x, y)
    /// * `color` - The color of the triangle
    pub fn draw_triangle(&self, p0: (u32, u32), p1: (u32, u32), p2: (u32, u32), color: Color) {
        let p0 = self.convert_point(p0, color);
        let p1 = self.convert_point(p1, color);
        let p2 = self.convert_point(p2, color);

        self.proxy
            .send_event(DrawEvent::Triangle(p0, p1, p2))
            .expect("Failed to send draw event");
    }
}
