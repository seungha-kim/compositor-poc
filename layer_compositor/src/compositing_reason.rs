use layer_model::*;

#[derive(Copy, Clone, Debug)]
pub enum CompositingReason {
    NewCoordSystem, // 정말?
}

pub fn get_compositing_reason(layer: &Layer) -> Option<CompositingReason> {
    match layer {
        Layer::Container(ref props) => reason_of_container(props),
        // TODO: 제대로
        _ => None,
    }
}

fn reason_of_container(_props: &ContainerProps) -> Option<CompositingReason> {
    return Some(CompositingReason::NewCoordSystem);
}
