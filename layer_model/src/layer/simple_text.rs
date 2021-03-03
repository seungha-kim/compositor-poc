use super::traits::*;
use crate::layer::common::*;

pub struct SimpleTextProps {
    pub content_rect: Rect,
    pub fill: Fill,
    pub opacity: f32,
    pub text: String,
}

impl DimensionLayer for SimpleTextProps {
    fn content_rect(&self) -> Rect {
        self.content_rect
    }

    fn effective_rect(&self) -> Rect {
        // FIXME: effective_rect 제대로
        self.content_rect
    }
}

impl TransparentLayer for SimpleTextProps {
    fn opacity(&self) -> f32 {
        self.opacity
    }
}
