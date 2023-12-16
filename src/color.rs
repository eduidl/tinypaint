//! Color module provides color representation and manipulation functionality.
//!
//! This module defines the `Color` struct and its associated methods for working with RGBA colors.

use bytemuck::{Pod, Zeroable};

/// Represents a color in RGBA format.
///
/// Each component (red, green, blue, alpha) is stored as a 32-bit floating-point number
/// in the range [0.0, 1.0].
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Color {
    /// Red component (0.0 to 1.0)
    pub r: f32,
    /// Green component (0.0 to 1.0)
    pub g: f32,
    /// Blue component (0.0 to 1.0)
    pub b: f32,
    /// Alpha component (0.0 to 1.0)
    pub a: f32,
}

impl Color {
    /// Pure black color (0.0, 0.0, 0.0, 1.0)
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    /// Pure red color (1.0, 0.0, 0.0, 1.0)
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    /// Pure green color (0.0, 1.0, 0.0, 1.0)
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    /// Pure blue color (0.0, 0.0, 1.0, 1.0)
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    /// Pure yellow color (1.0, 1.0, 0.0, 1.0)
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
    /// Pure magenta color (1.0, 0.0, 1.0, 1.0)
    pub const MAGENTA: Self = Self::rgb(1.0, 0.0, 1.0);
    /// Pure cyan color (0.0, 1.0, 1.0, 1.0)
    pub const CYAN: Self = Self::rgb(0.0, 1.0, 1.0);
    /// Pure white color (1.0, 1.0, 1.0, 1.0)
    pub const WHITE: Self = Self::rgba(1.0, 1.0, 1.0, 1.0);

    /// Creates a new color with the specified RGB components and full opacity.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0.0 to 1.0)
    /// * `g` - Green component (0.0 to 1.0)
    /// * `b` - Blue component (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// A new `Color` instance with alpha set to 1.0
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a new color with the specified RGBA components.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0.0 to 1.0)
    /// * `g` - Green component (0.0 to 1.0)
    /// * `b` - Blue component (0.0 to 1.0)
    /// * `a` - Alpha component (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// A new `Color` instance with the specified components
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}
