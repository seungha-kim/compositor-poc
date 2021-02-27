use super::traits::*;
use crate::layer::common::*;

pub struct ContainerProps {
    pub content_rect: Rect,
    pub border: Option<Border>,
    pub fill: Option<Fill>,
    pub opacity: f32,
    pub children: Vec<LayerId>,
}

impl DimensionLayer for ContainerProps {
    fn content_rect(&self) -> Rect {
        self.content_rect
    }

    fn effective_rect(&self) -> Rect {
        self.content_rect
    }
}

impl BorderLayer for ContainerProps {
    fn border(&self) -> Option<Border> {
        self.border
    }
}

impl TransparentLayer for ContainerProps {
    fn opacity(&self) -> f32 {
        self.opacity
    }
}

impl FillableLayer for ContainerProps {
    fn fill(&self) -> Option<Fill> {
        self.fill
    }
}

pub enum ContainerUpdateCommand {}
