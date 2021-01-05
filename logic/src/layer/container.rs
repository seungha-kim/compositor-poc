use crate::layer::common::*;

pub struct ContainerProps {
    pub content_rect: Rect,
    pub border: Option<Border>,
    pub fill: Option<Fill>,
    pub opacity: f32,
    pub children: Vec<LayerId>,
}

pub enum ContainerUpdateCommand {}
