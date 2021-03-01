use futures::executor::block_on;
use layer_model::commands::LayerCreationCommand;
use layer_model::rect::RectProps;
use layer_model::Layer::Container;
use layer_model::*;
use raqote::SolidSource;
use wgpu_renderer::wgpu_layer::*;
use winit::window::Window;
use winit::{
    dpi::{LogicalSize, Size},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub trait SceneLogic {
    fn initialize_scene(&mut self, layer_repo: &mut LayerRepository);
    fn handle_window_event(
        &mut self,
        window_event: &WindowEvent,
        layer_repo: &mut LayerRepository,
    ) -> ControlFlow;
    fn update(&mut self, layer_repo: &mut LayerRepository);
}

pub struct SceneController<L: SceneLogic> {
    pub window: Window,
    quad_renderer: QuadRenderer,
    pub layer_repository: LayerRepository,
    // TODO: 지금은 텍스처 하나에 전부 다 그리지만, 개선되어야 함
    root_quad_id: QuadId,
    root_draw_target: raqote::DrawTarget,
    pub logic: L,
}

impl<L: SceneLogic> SceneController<L> {
    pub fn new(event_loop: &EventLoop<()>, width: f64, height: f64, logic: L) -> Self {
        let window = WindowBuilder::new()
            .with_inner_size(Size::Logical(LogicalSize::new(width, height)))
            .build(&event_loop)
            .unwrap();
        let mut logic = logic;
        let width = window.inner_size().width as f32;
        let height = window.inner_size().height as f32;
        let mut quad_renderer = block_on(QuadRenderer::new(&window));
        let root_quad_id = quad_renderer.new_quad(width / -2.0, height / -2.0, width, height);
        let mut layer_repository =
            LayerRepository::new(layer_model::common::Size::new(width, height));
        let root_draw_target = raqote::DrawTarget::new(width as i32, height as i32);
        logic.initialize_scene(&mut layer_repository);
        SceneController {
            window,
            quad_renderer,
            root_quad_id,
            root_draw_target,
            layer_repository,
            logic,
        }
    }

    pub fn update(&mut self) {
        self.logic.update(&mut self.layer_repository);
        self.quad_renderer.update();
    }

    pub fn render(&mut self) {
        // TODO: 매번 clear 하지 말아야 함
        self.root_draw_target.clear(SolidSource {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });
        layer_renderer::render_scene(&self.layer_repository, &mut self.root_draw_target);
        let image = self.root_draw_target.get_data_u8().to_vec();
        self.quad_renderer
            .update_texture(self.root_quad_id, image.as_slice());
        match self.quad_renderer.render() {
            Ok(_) => {}
            Err(RendererError::SwapChainLost) => self.quad_renderer.resize(self.quad_renderer.size),
            // Err(RendererError::SwapChainOutOfMemory) => *control_flow = ControlFlow::Exit,
            Err(e) => eprintln!("{:?}", e),
        }
    }

    fn handle_resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.quad_renderer.resize(size);
    }
}
