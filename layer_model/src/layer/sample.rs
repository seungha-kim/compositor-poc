use super::traits::*;
use crate::layer::common::*;

pub struct SampleLayerProps {
    pub content_rect: Rect,
    pub border: Option<Border>,
    pub fill: Option<Fill>,
    pub opacity: f32,
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
    fn fill(&self) -> Option<&Fill> {
        self.fill.as_ref()
    }
}

impl BorderLayer for SampleLayerProps {
    fn border(&self) -> Option<&Border> {
        self.border.as_ref()
    }
}

impl TransparentLayer for SampleLayerProps {
    fn opacity(&self) -> f32 {
        self.opacity
    }
}

pub struct SampleLayerCreationCommand(pub Rect);
pub enum SampleLayerUpdateCommand {}
