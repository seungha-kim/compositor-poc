use super::composite::*;
use super::compositing_reason::*;
use crate::rasterizer::rasterize_container_layer;
use logic::*;

struct Compositor {
    size: Size,
    scale_factor: f32,
}

// 설계 TODO: raquote::DrawTarget::push_layer를 쓸 수도 있을 것 같은데...
// 근데 이러면 여기서 CachingReason 까지 다 판별해야 함 (어디서 push_layer 를 할지를 여기서 판단할 수 있어야 되기 때문에)
// 뭐 그것도 좋긴 하겠지... renderer 는 판단같은거 안 하고 진짜로 렌더링만 하면 될테니까
// 근데 귀찮으면 그냥 이대로 가도 될 것 같기도 하고... push_layer 이것도 내부적으로 DrawTarget 비슷한 거 만들 것 같다.

impl Compositor {
    fn new() -> Self {
        Self {
            size: Size::new(0.0, 0.0),
            scale_factor: 1.0,
        }
    }

    pub fn synchronize_composites(
        &mut self,
        layer_repo: &LayerRepository,
        composite_repo: &mut CompositeRepository,
    ) {
        let root_layer_id = layer_repo.root_layer_id().clone();
        let root_composite_id = composite_repo.root_composite_id().clone();
        self.visit(
            layer_repo,
            composite_repo,
            &root_layer_id,
            &root_composite_id,
        );
    }

    fn visit(
        &mut self,
        layer_repo: &LayerRepository,
        composite_repo: &mut CompositeRepository,
        layer_id: &LayerId,
        composite_id: &CompositeId,
    ) {
        match layer_repo.get_layer_by_id(layer_id) {
            Layer::Container(ref props) => {
                self.visit_container_layer(layer_repo, composite_repo, props, composite_id)
            }
        }
    }

    fn visit_container_layer(
        &mut self,
        layer_repo: &LayerRepository,
        composite_repo: &mut CompositeRepository,
        props: &ContainerProps,
        parent_composite_id: &CompositeId,
    ) {
        let composite = composite_repo.get_composite_by_id_mut(parent_composite_id);
        rasterize_container_layer(composite, props);

        let mut child_comp_idx = 0;
        for child_layer_idx in 0..props.children.len() {
            let child_layer_id = &props.children[child_layer_idx];
            let child_layer = layer_repo.get_layer_by_id(child_layer_id);
            if let Some(compositing_reason) = get_compositing_reason(child_layer) {
                // TODO: reorder / delete layer
                if composite_repo.create_nth_child_if_not_exists(
                    parent_composite_id,
                    child_comp_idx,
                    child_layer_id,
                    compositing_reason,
                ) {
                    child_comp_idx += 1;
                }
            }
            // TODO: update composite.rect.origin
        }
    }
}
