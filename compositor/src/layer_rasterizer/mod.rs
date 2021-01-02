use crate::layer::*;
pub use raqote::*;

pub fn rasterize_container_layer(draw_target: &mut DrawTarget, layer: &ContainerLayer) {
    if let Some(Fill::Color { r, g, b, a }) = layer.fill() {
        let mut fill_pb = PathBuilder::new();
        let fill_size = layer.content_rect().size;
        fill_pb.move_to(0., 0.);
        fill_pb.line_to(fill_size.width, 0.);
        fill_pb.line_to(fill_size.width, fill_size.height);
        fill_pb.line_to(0., fill_size.height);
        let fill_path = fill_pb.finish();
        let fill_source = Source::Solid(SolidSource { r, g, b, a });
        draw_target.fill(&fill_path, &fill_source, &DrawOptions::new());
    }
    // TODO: border
}
