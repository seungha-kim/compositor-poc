use super::common::*;
use super::container::*;

pub enum Layer {
    Container(ContainerProps),
}

impl Default for Layer {
    fn default() -> Self {
        Layer::Container(ContainerProps {
            opacity: 1.0,
            content_rect: Rect {
                size: Size::new(0.0, 0.0),
                origin: Point::new(0.0, 0.0),
            },
            border: None,
            fill: None,
            children: Vec::new(),
        })
    }
}
