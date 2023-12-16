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
    end_index: u32,
}

impl Command {
    fn new(primitive: DrawPrimitive, begin_index: u32) -> Self {
        Self {
            primitive,
            begin_index,
            end_index: begin_index + primitive.vertex_count(),
        }
    }

    fn inc(&mut self) {
        self.end_index += self.primitive.vertex_count();
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

#[derive(Debug)]
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
        let point_pipeline =
            Self::create_pipeline(device, surface_format, wgpu::PrimitiveTopology::PointList);
        let line_pipeline =
            Self::create_pipeline(device, surface_format, wgpu::PrimitiveTopology::LineList);
        let triangle_pipeline = Self::create_pipeline(
            device,
            surface_format,
            wgpu::PrimitiveTopology::TriangleList,
        );

        Self {
            commands: Vec::new(),
            vertices: Vec::new(),
            buffer: None,
            point_pipeline,
            line_pipeline,
            triangle_pipeline,
        }
    }

    pub fn enque_point(&mut self, p0: Vertex) {
        match self.commands.last() {
            Some(command) if command.primitive == DrawPrimitive::Point => {
                self.commands.last_mut().unwrap().inc();
            }
            _ => {
                self.commands
                    .push(Command::point(self.vertices.len() as u32));
            }
        }

        self.vertices.push(p0);
    }

    pub fn enque_line(&mut self, p0: Vertex, p1: Vertex) {
        match self.commands.last() {
            Some(command) if command.primitive == DrawPrimitive::Line => {
                self.commands.last_mut().unwrap().inc();
            }
            _ => {
                self.commands
                    .push(Command::line(self.vertices.len() as u32));
            }
        }

        self.vertices.push(p0);
        self.vertices.push(p1);
    }

    pub fn enque_triangle(&mut self, p0: Vertex, p1: Vertex, p2: Vertex) {
        match self.commands.last() {
            Some(command) if command.primitive == DrawPrimitive::Triangle => {
                self.commands.last_mut().unwrap().inc();
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
            render_pass.draw(command.begin_index..command.end_index, 0..1);
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
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        })
    }

    fn shader(device: &wgpu::Device) -> wgpu::ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("[tinypaint] Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        })
    }
}
