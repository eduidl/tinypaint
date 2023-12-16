use tinypaint::{draw, Color};

#[pollster::main]
async fn main() {
    draw(|ctx| {
        ctx.draw_point((20, 20), Color::RED);
        ctx.draw_line((20, 20), (100, 100), Color::GREEN);
    })
    .await;
}
