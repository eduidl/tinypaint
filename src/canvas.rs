//! Canvas module provides the main window and rendering context for the painting application.
//!
//! This module handles window creation, event loop management, and rendering operations.

use std::sync::Arc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder},
};

use crate::{
    context::CanvasContext,
    error::{Result, TinyPaintError},
    renderer::Renderer,
    vertex::DrawEvent,
};

/// Represents the main canvas window and rendering context.
///
/// The `Canvas` struct manages the window, event loop, and renderer for the painting application.
/// It provides methods to create a new canvas, get the drawing context, and run the main event loop.
pub struct Canvas {
    window: Arc<Window>,
    event_loop: Option<EventLoop<DrawEvent>>,
    renderer: Renderer<'static>,
}

impl Canvas {
    /// Creates a new canvas with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the canvas in pixels
    /// * `height` - The height of the canvas in pixels
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `Canvas` instance
    pub async fn new(width: u32, height: u32) -> Result<Self> {
        let event_loop = EventLoopBuilder::<DrawEvent>::with_user_event()
            .build()
            .map_err(TinyPaintError::EventLoopError)?;
        let window = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .with_resizable(false)
            .build(&event_loop)
            .map_err(TinyPaintError::WindowOsError)?;
        let window = Arc::new(window);

        let renderer = Renderer::new(Arc::clone(&window)).await?;

        Ok(Self {
            window,
            event_loop: Some(event_loop),
            renderer,
        })
    }

    /// Returns a context for drawing operations
    pub fn context(&self) -> CanvasContext {
        let size = self.window.inner_size();
        CanvasContext::new(
            size.width,
            size.height,
            self.event_loop.as_ref().unwrap().create_proxy(),
        )
    }

    /// Runs the main event loop
    pub fn run(mut self) -> Result<()> {
        let event_loop = self.event_loop.take().unwrap();

        event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent { window_id, event } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }
                        WindowEvent::RedrawRequested => match self.renderer.render() {
                            Ok(_) => (),
                            Err(TinyPaintError::SurfaceError(wgpu::SurfaceError::Lost)) => {
                                self.renderer.reconfigure();
                            }
                            Err(TinyPaintError::SurfaceError(wgpu::SurfaceError::OutOfMemory)) => {
                                elwt.exit();
                            }
                            Err(e) => log::error!("{:?}", e),
                        },
                        _ => (),
                    }
                }
                Event::UserEvent(e) => {
                    self.renderer.handle_event(e);
                }
                Event::AboutToWait => {
                    self.window.request_redraw();
                }
                _ => (),
            })
            .map_err(crate::error::TinyPaintError::from)
    }
}
