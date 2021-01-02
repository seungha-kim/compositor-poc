use compositor::layer_rasterizer::{rasterize_container_layer, DrawTarget, Transform};
use compositor::*;

fn main() {
    println!("Hello, world!");
    let container_layer = ContainerLayer::new(10., 10., 30., 30.);
    let mut draw_target = DrawTarget::new(500, 500);
    let container_layer_origin = container_layer.content_rect().origin;
    draw_target.set_transform(&Transform::create_translation(
        container_layer_origin.x,
        container_layer_origin.y,
    ));
    rasterize_container_layer(&mut draw_target, &container_layer);
    draw_target.write_png("result.png");
}
