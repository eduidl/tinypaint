use crate::vertex::Vertex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DrawPrimitive {
    Point,
    Line,
    Triangle,
}

impl DrawPrimitive {
    const fn vertex_count(self) -> u32 {
        match self {
            Self::Point => 1,
            Self::Line => 2,
            Self::Triangle => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Command {
    primitive: DrawPrimitive,
    begin_index: u32,
    count: u32,
}

impl Command {
    fn new(primitive: DrawPrimitive, begin_index: u32) -> Self {
        Self {
            primitive,
            begin_index,
            count: 1,
        }
    }

    fn end_index(&self) -> u32 {
        self.begin_index + self.count * self.primitive.vertex_count()
    }

    fn inc(&mut self) {
        self.count += 1;
    }

    fn point(begin_index: u32) -> Self {
        Self::new(DrawPrimitive::Point, begin_index)
    }

    fn line(begin_index: u32) -> Self {
        Self::new(DrawPrimitive::Line, begin_index)
    }

    fn triangle(begin_index: u32) -> Self {
        Self::new(DrawPrimitive::Triangle, begin_index)
    }
}

pub(crate) struct Commands {
    commands: Vec<Command>,
    vertices: Vec<Vertex>,
    buffer: Option<wgpu::Buffer>,
    point_pipeline: wgpu::RenderPipeline,
    line_pipeline: wgpu::RenderPipeline,
    triangle_pipeline: wgpu::RenderPipeline,
}

impl Commands {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        Self {
            commands: Vec::new(),
            vertices: Vec::new(),
            buffer: None,
            point_pipeline: Self::create_pipeline(
                device,
                surface_format,
                wgpu::PrimitiveTopology::PointList,
            ),
            line_pipeline: Self::create_pipeline(
                device,
                surface_format,
                wgpu::PrimitiveTopology::LineList,
            ),
            triangle_pipeline: Self::create_pipeline(
                device,
                surface_format,
                wgpu::PrimitiveTopology::TriangleList,
            ),
        }
    }

    pub fn enqueue_point(&mut self, p0: Vertex) {
        match self.commands.last_mut() {
            Some(command) if command.primitive == DrawPrimitive::Point => {
                command.inc();
            }
            _ => {
                self.commands
                    .push(Command::point(self.vertices.len() as u32));
            }
        }

        self.vertices.push(p0);
    }

    pub fn enqueue_line(&mut self, p0: Vertex, p1: Vertex) {
        match self.commands.last_mut() {
            Some(command) if command.primitive == DrawPrimitive::Line => {
                command.inc();
            }
            _ => {
                self.commands
                    .push(Command::line(self.vertices.len() as u32));
            }
        }

        self.vertices.push(p0);
        self.vertices.push(p1);
    }

    pub fn enqueue_triangle(&mut self, p0: Vertex, p1: Vertex, p2: Vertex) {
        match self.commands.last_mut() {
            Some(command) if command.primitive == DrawPrimitive::Triangle => {
                command.inc();
            }
            _ => {
                self.commands
                    .push(Command::triangle(self.vertices.len() as u32));
            }
        }

        self.vertices.push(p0);
        self.vertices.push(p1);
        self.vertices.push(p2);
    }

    pub fn prepare(&mut self, device: &wgpu::Device) {
        self.buffer = Some(Vertex::buffer(device, &self.vertices));
    }

    pub fn render<'rpass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'rpass>) {
        render_pass.set_vertex_buffer(0, self.buffer.as_ref().unwrap().slice(..));

        for command in &self.commands {
            let pipeline = match command.primitive {
                DrawPrimitive::Point => &self.point_pipeline,
                DrawPrimitive::Line => &self.line_pipeline,
                DrawPrimitive::Triangle => &self.triangle_pipeline,
            };
            render_pass.set_pipeline(pipeline);
            render_pass.draw(command.begin_index..command.end_index(), 0..1);
        }
    }

    fn create_pipeline(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        topology: wgpu::PrimitiveTopology,
    ) -> wgpu::RenderPipeline {
        let shader = Self::shader(device);

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("[tinypaint] Render Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
    }

    fn shader(device: &wgpu::Device) -> wgpu::ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("[tinypaint] Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        })
    }
}
