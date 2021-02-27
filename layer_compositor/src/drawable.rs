use layer_model::*;

pub trait Drawable {
    fn content_rect(&self) -> Rect;
    fn effective_margin(&self) -> SideOffsets;
    fn opacity(&self) -> f32;
    fn is_opaque(&self) -> bool {
        self.opacity() >= 1.0
    }
}

impl Drawable for Layer {
    fn content_rect(&self) -> Rect {
        use layer_model::Layer as t;

        match self {
            t::Container(ref data) => data.content_rect,
        }
    }

    fn effective_margin(&self) -> SideOffsets {
        // TODO: border, shadow, ...
        unimplemented!()
    }

    fn opacity(&self) -> f32 {
        use layer_model::Layer as t;

        match self {
            t::Container(ref data) => data.opacity,
        }
    }
}
