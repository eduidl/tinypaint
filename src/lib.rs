mod canvas;
mod color;
mod command;
mod context;
mod vertex;

pub use canvas::Canvas;
pub use color::Color;
pub use context::CanvasContext;

pub async fn draw(func: impl Fn(CanvasContext) + Send + 'static) {
    let canvas = Canvas::new(800, 600).await;
    let context = canvas.context();

    std::thread::spawn(move || func(context));

    canvas.run();
}
