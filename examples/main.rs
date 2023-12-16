use tinypaint::{draw, Color};

#[pollster::main]
async fn main() {
    draw(800, 600, |ctx| {
        ctx.draw_point((20, 20), Color::RED);
        ctx.draw_line((20, 20), (700, 500), Color::GREEN);
        ctx.draw_triangle((20, 20), (600, 500), (400, 200), Color::BLUE);
    })
    .await
    .unwrap();
}
