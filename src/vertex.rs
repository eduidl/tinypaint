//! Vertex module provides vertex data structures and utilities for rendering.
//!
//! This module defines the `Vertex` struct for representing vertices with position and color,
//! and the `DrawEvent` enum for different types of drawing operations.

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::Color;

/// Represents a vertex with position and color attributes.
///
/// This struct is used for rendering shapes and is compatible with WGPU's vertex buffer layout.
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub(crate) struct Vertex {
    /// The position of the vertex in normalized device coordinates
    pub position: [f32; 2],
    /// The color of the vertex
    pub color: Color,
}

impl Vertex {
    /// The vertex attributes for the vertex buffer layout
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x4,
    ];

    /// Returns the vertex buffer layout description.
    ///
    /// This is used to configure the vertex buffer layout for the WGPU pipeline.
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Vertex::ATTRIBUTES,
        }
    }

    /// Creates a new vertex buffer from a slice of vertices.
    ///
    /// # Arguments
    ///
    /// * `device` - The WGPU device
    /// * `vertices` - A slice of vertices to create the buffer from
    ///
    /// # Returns
    ///
    /// A new vertex buffer containing the provided vertices
    pub fn buffer(device: &wgpu::Device, vertices: &[Self]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("[tinypaint] Vertex Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(vertices),
        })
    }
}

/// Represents different types of drawing events.
///
/// This enum is used to communicate drawing operations between the canvas context
/// and the renderer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum DrawEvent {
    /// Draw a single point
    Point(Vertex),
    /// Draw a line between two points
    Line(Vertex, Vertex),
    /// Draw a triangle defined by three points
    Triangle(Vertex, Vertex, Vertex),
}

/// Represents a 2D point with x and y coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// The x coordinate
    pub x: f32,
    /// The y coordinate
    pub y: f32,
}
