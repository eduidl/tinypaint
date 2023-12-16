use winit::event_loop::EventLoopProxy;

use crate::{
    vertex::{DrawEvent, Vertex},
    Color,
};

fn convert_x(x: u32, max: u32) -> f32 {
    (x * 2) as f32 / max as f32 - 1.0
}

fn convert_y(y: u32, max: u32) -> f32 {
    1.0 - (y * 2) as f32 / max as f32
}

#[derive(Debug)]
pub struct CanvasContext {
    width: u32,
    height: u32,
    proxy: EventLoopProxy<DrawEvent>,
}

impl CanvasContext {
    pub(crate) fn new(width: u32, height: u32, proxy: EventLoopProxy<DrawEvent>) -> Self {
        Self {
            width,
            height,
            proxy,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn draw_point(&self, p0: (u32, u32), color: Color) {
        let create_point = |p: (u32, u32)| Vertex {
            position: [convert_x(p.0, self.width), convert_y(p0.1, self.height)],
            color,
        };

        let p0 = create_point(p0);

        self.proxy.send_event(DrawEvent::Point(p0)).unwrap();
    }

    pub fn draw_line(&self, p0: (u32, u32), p1: (u32, u32), color: Color) {
        let create_point = |p: (u32, u32)| Vertex {
            position: [convert_x(p.0, self.width), convert_y(p0.1, self.height)],
            color,
        };

        let p0 = create_point(p0);
        let p1 = create_point(p1);

        self.proxy.send_event(DrawEvent::Line(p0, p1)).unwrap();
    }

    pub fn draw_triangle(&self, p0: (u32, u32), p1: (u32, u32), p2: (u32, u32), color: Color) {
        let create_point = |p: (u32, u32)| Vertex {
            position: [convert_x(p.0, self.width), convert_y(p0.1, self.height)],
            color,
        };

        let p0 = create_point(p0);
        let p1 = create_point(p1);
        let p2 = create_point(p2);

        self.proxy
            .send_event(DrawEvent::Triangle(p0, p1, p2))
            .unwrap();
    }
}
