use super::traits::*;
use crate::layer::common::*;

pub struct SampleLayerProps {
    content_rect: Rect,
    border: Option<Border>,
    fill: Option<Fill>,
    opacity: f32,
}

impl DimensionLayer for SampleLayerProps {
    fn content_rect(&self) -> Rect {
        self.content_rect
    }

    fn effective_rect(&self) -> Rect {
        // FIXME: effective_rect 제대로
        self.content_rect
    }
}

impl FillableLayer for SampleLayerProps {
    fn fill(&self) -> Option<Fill> {
        self.fill
    }
}

impl BorderLayer for SampleLayerProps {
    fn border(&self) -> Option<Border> {
        self.border
    }
}

impl TransparentLayer for SampleLayerProps {
    fn opacity(&self) -> f32 {
        self.opacity
    }
}

pub struct SampleLayerCreationCommand(Rect);
pub enum SampleLayerUpdateCommand {}
