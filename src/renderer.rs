use std::sync::Arc;

use winit::window::Window;

use crate::{command::Commands, error::Result, vertex::DrawEvent, TinyPaintError};

pub(crate) struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    commands: Commands,
}

impl Renderer<'_> {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let instance = wgpu::Instance::default();

        let surface = instance
            .create_surface(Arc::clone(&window))
            .map_err(TinyPaintError::CreateSurfaceError)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(TinyPaintError::RequestAdapterError)?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .map_err(TinyPaintError::RequestDeviceError)?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let commands = Commands::new(&device, surface_format);

        Ok(Self {
            surface,
            surface_config,
            device,
            queue,
            commands,
        })
    }

    pub fn handle_event(&mut self, event: DrawEvent) {
        match event {
            DrawEvent::Point(p0) => {
                self.commands.enqueue_point(p0);
            }
            DrawEvent::Line(p0, p1) => {
                self.commands.enqueue_line(p0, p1);
            }
            DrawEvent::Triangle(p0, p1, p2) => {
                self.commands.enqueue_triangle(p0, p1, p2);
            }
        }
    }

    pub fn reconfigure(&self) {
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self) -> Result<()> {
        let output = self
            .surface
            .get_current_texture()
            .map_err(TinyPaintError::SurfaceError)?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.commands.prepare(&self.device);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("[tinypaint] Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.commands.render(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
