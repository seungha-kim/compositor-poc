use crate::compositing_reason::*;
use layer_model::*;
use raqote as r;
use std::collections::HashMap;

pub type CompositeId = usize;

pub struct CompositeRepository {
    id_count: CompositeId,
    composite_map: HashMap<CompositeId, Composite>,
    root_composite_id: CompositeId,
}

pub struct Composite {
    pub source_layer_id: LayerId,
    pub compositing_reason: CompositingReason,
    pub children: Vec<CompositeId>,
    pub rect: Rect,
    pub draw_target: Option<Box<r::DrawTarget>>,
}

impl CompositeRepository {
    pub fn root_composite_id(&self) -> &CompositeId {
        &self.root_composite_id
    }

    pub fn get_composite_by_id(&self, id: &CompositeId) -> &Composite {
        self.composite_map.get(id).unwrap()
    }

    pub fn get_composite_by_id_mut(&mut self, id: &CompositeId) -> &mut Composite {
        self.composite_map.get_mut(id).unwrap()
    }

    pub fn new_composite(&mut self, composite: Composite) -> CompositeId {
        self.id_count += 1;
        self.composite_map.insert(self.id_count, composite);
        self.id_count
    }

    pub fn create_nth_child_if_not_exists(
        &mut self,
        parent_composite_id: &CompositeId,
        nth: usize,
        source_layer_id: &LayerId,
        compositing_reason: CompositingReason,
    ) -> bool {
        let have_no_matching_composite = self
            .get_composite_by_id(parent_composite_id)
            .children
            .get(nth)
            .is_none();
        if have_no_matching_composite {
            let child_composite_id = self.new_composite(Composite {
                source_layer_id: *source_layer_id,
                compositing_reason,
                children: vec![],
                rect: Default::default(),
                draw_target: None,
            });
            let parent_composite = self.get_composite_by_id_mut(parent_composite_id);
            parent_composite.children.push(child_composite_id);
            // TODO: reorder / delete layer
        }
        have_no_matching_composite
    }
}
