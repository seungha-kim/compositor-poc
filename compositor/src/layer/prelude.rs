pub type Rect = euclid::default::Rect<f32>;
pub type SideOffsets = euclid::default::SideOffsets2D<f32>;

#[derive(Copy, Clone, Debug)]
pub enum BorderPosition {
    Inner,
    Center,
    Outer,
}

#[derive(Copy, Clone, Debug)]
pub struct Border {
    pub position: BorderPosition,
    pub fill: Fill,
    pub width: f32,
}

#[derive(Copy, Clone, Debug)]
pub enum Fill {
    Color { r: u8, g: u8, b: u8, a: u8 },
}

pub trait Layer {
    fn content_rect(&self) -> Rect;
    fn effective_margin(&self) -> SideOffsets;
    fn border(&self) -> Option<Border>;
    fn fill(&self) -> Option<Fill>;
    fn opacity(&self) -> f32;

    fn is_opaque(&self) -> bool {
        self.opacity() == 1.0
    }
}
