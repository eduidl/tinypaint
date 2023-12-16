use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder},
};

use crate::{command::Commands, context::CanvasContext, vertex::DrawEvent};

pub struct Canvas {
    window: Window,
    event_loop: Option<EventLoop<DrawEvent>>,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    commands: Commands,
}

impl Canvas {
    pub async fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoopBuilder::<DrawEvent>::with_user_event().build();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

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
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        let commands = Commands::new(&device, surface_format);

        Self {
            window,
            event_loop: Some(event_loop),
            surface,
            surface_config,
            device,
            queue,
            commands,
        }
    }

    pub fn context(&self) -> CanvasContext {
        let size = self.window.inner_size();
        CanvasContext::new(
            size.width,
            size.height,
            self.event_loop.as_ref().unwrap().create_proxy(),
        )
    }

    pub fn run(mut self) -> ! {
        let event_loop = self.event_loop.take().unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::UserEvent(e) => match e {
                    DrawEvent::Point(p0) => {
                        self.commands.enque_point(p0);
                    }
                    DrawEvent::Line(p0, p1) => {
                        self.commands.enque_line(p0, p1);
                    }
                    DrawEvent::Triangle(p0, p1, p2) => {
                        self.commands.enque_triangle(p0, p1, p2);
                    }
                },
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.id() => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    match self.render() {
                        Ok(_) => (),
                        Err(wgpu::SurfaceError::Lost) => {
                            self.surface.configure(&self.device, &self.surface_config);
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            *control_flow = ControlFlow::Exit;
                        }
                        Err(e) => log::error!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => (),
            }
        })
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.commands.prepare(&self.device);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("[tinypaint] Render Encoder"),
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
                ..Default::default()
            });

            self.commands.render(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
