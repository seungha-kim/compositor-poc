use crate::composite::*;
use logic::*;
pub use raqote::*;
use std::convert::TryInto;

pub fn rasterize_container_layer(composite: &mut Composite, props: &ContainerProps) {
    // TODO: effective_size
    let mut draw_target = DrawTarget::new(
        props.content_rect.size.width as i32,
        props.content_rect.size.height as i32,
    );
    if let Some(Fill::Color { r, g, b, a }) = props.fill {
        let mut fill_pb = PathBuilder::new();
        let fill_size = props.content_rect.size;
        fill_pb.move_to(0., 0.);
        fill_pb.line_to(fill_size.width, 0.);
        fill_pb.line_to(fill_size.width, fill_size.height);
        fill_pb.line_to(0., fill_size.height);
        let fill_path = fill_pb.finish();
        let fill_source = Source::Solid(SolidSource { r, g, b, a });
        draw_target.fill(&fill_path, &fill_source, &DrawOptions::new());
    }
    composite.draw_target = Some(Box::new(draw_target));
    // TODO: border
}
