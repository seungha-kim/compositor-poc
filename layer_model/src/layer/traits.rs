use super::common::*;

pub trait FillableLayer {
    fn fill(&self) -> Option<&Fill>;
}

pub trait BorderLayer {
    fn border(&self) -> Option<&Border>;
}

pub trait TransparentLayer {
    fn opacity(&self) -> f32;
    fn is_opaque(&self) -> bool {
        self.opacity() >= 1.0
    }
}

pub trait DimensionLayer {
    fn content_rect(&self) -> Rect;
    fn effective_rect(&self) -> Rect;
}
