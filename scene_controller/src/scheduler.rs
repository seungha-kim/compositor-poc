// use layer_animator::Animator;
// use layer_model::layer_compositor::Compositor as CompositorTrait;
// use layer_model::layer::common::LayerTodo;
// use layer_model::ContainerLayer;
//
// pub struct SceneController<C>
// where
//     C: CompositorTrait,
// {
//     layer_compositor: C,
//     root_layer: ContainerLayer,
//     layer_animator: Animator,
// }
//
// impl<C> SceneController<C>
// where
//     C: CompositorTrait,
// {
//     pub fn handle_event(&mut self) {
//         // TODO
//     }
//
//     pub fn update(&mut self) {
//         // TODO
//         // self.layer_animator.step();
//     }
//
//     pub fn render(&mut self) {
//         // layer1 레벨의 dirty 플래그 켜져 있음
//         self.root_layer.compose(&mut self.layer_compositor);
//         // TODO: layer1 레벨의 dirty 플래그 끄기 (여기에서든, 안에서 돌면서든)
//     }
// }
