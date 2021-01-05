use crate::layer::*;

pub type Size = euclid::default::Size2D<f32>;
pub type Point = euclid::default::Point2D<f32>;
pub type Rect = euclid::default::Rect<f32>;
pub type SideOffsets = euclid::default::SideOffsets2D<f32>;

pub type LayerId = String;

#[derive(Copy, Clone, Debug)]
pub struct CompositingFlag {
    pub needs_paint: bool,
    pub needs_update_transform: bool,
}

impl CompositingFlag {
    pub fn clear(&mut self) {
        self.needs_paint = false;
        self.needs_update_transform = false;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CachingReason {
    Whatever,
}

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
