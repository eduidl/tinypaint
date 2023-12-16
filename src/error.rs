use thiserror::Error;

/// Represents all possible errors that can occur in TinyPaint
#[derive(Error, Debug)]
pub enum TinyPaintError {
    /// Error from winit event loop creation
    #[error("Failed to create event loop: {0}")]
    EventLoopError(#[from] winit::error::EventLoopError),

    /// Error from winit window creation (OS error)
    #[error("Failed to create window: {0}")]
    WindowOsError(#[from] winit::error::OsError),

    /// Error from surface creation
    #[error("Failed to create surface: {0}")]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),

    /// Error from WGPU initialization
    #[error("Failed to initialize WGPU: {0}")]
    RequestAdapterError(#[from] wgpu::RequestAdapterError),

    /// Error to get current surface texture
    #[error("Failed to get surface: {0}")]
    SurfaceError(#[from] wgpu::SurfaceError),

    /// Error from device creation
    #[error("Failed to create device: {0}")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),
}

/// A type alias for `Result<T, TinyPaintError>`
pub type Result<T> = std::result::Result<T, TinyPaintError>;
