use super::common::*;
use super::container::*;
use crate::layer::rect::RectProps;
use crate::SampleLayerProps;

pub enum Layer {
    Container(ContainerProps),
    Rect(RectProps),
    Sample(SampleLayerProps),
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
