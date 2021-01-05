use animator::Animator;
use logic::compositor::Compositor as CompositorTrait;
use logic::layer::common::LayerTodo;
use logic::ContainerLayer;

pub struct Scheduler<C>
where
    C: CompositorTrait,
{
    compositor: C,
    root_layer: ContainerLayer,
    animator: Animator,
}

impl<C> Scheduler<C>
where
    C: CompositorTrait,
{
    pub fn handle_event(&mut self) {
        // TODO
    }

    pub fn update(&mut self) {
        // TODO
        // self.animator.step();
    }

    pub fn render(&mut self) {
        // layer1 레벨의 dirty 플래그 켜져 있음
        self.root_layer.compose(&mut self.compositor);
        // TODO: layer1 레벨의 dirty 플래그 끄기 (여기에서든, 안에서 돌면서든)
    }
}
