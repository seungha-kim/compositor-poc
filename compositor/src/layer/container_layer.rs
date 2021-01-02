use super::prelude::*;
use euclid::{Point2D, Size2D};

pub struct ContainerLayer {
    content_rect: Rect,
    border: Option<Border>,
    fill: Option<Fill>,
    opacity: f32,
    children: Vec<Box<dyn Layer>>,
}

impl ContainerLayer {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            content_rect: Rect::new(Point2D::new(x, y), Size2D::new(width, height)),
            // TODO: fill - 아니면 이거 그냥 public으로 하고 default 제공해도..?
            fill: Some(Fill::Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }),
            children: Vec::new(),
            opacity: 1.0,
            // TODO: border
            border: Some(Border {
                position: BorderPosition::Inner,
                fill: Fill::Color {
                    r: 100,
                    g: 100,
                    b: 100,
                    a: 255,
                },
                width: 2.0,
            }),
        }
    }

    pub fn children(&self) -> &[Box<dyn Layer>] {
        &self.children
    }

    pub fn push_child(&mut self, child: Box<dyn Layer>) {
        self.children.push(child);
    }
}

impl Layer for ContainerLayer {
    fn content_rect(&self) -> Rect {
        self.content_rect
    }

    fn effective_margin(&self) -> SideOffsets {
        unimplemented!()
    }

    fn border(&self) -> Option<Border> {
        self.border
    }

    fn fill(&self) -> Option<Fill> {
        self.fill
    }

    fn opacity(&self) -> f32 {
        self.opacity
    }
}
