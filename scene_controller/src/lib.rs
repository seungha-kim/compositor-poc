use futures::executor::block_on;
use layer_model::commands::LayerCreationCommand;
use layer_model::*;
use wgpu_renderer::wgpu_layer::*;
use winit::window::Window;
use winit::{
    dpi::{LogicalSize, Size},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn main() {
    let initial_width = 300.;
    let initial_height = 300.;

    env_logger::init();
    let event_loop = EventLoop::new();

    let mut scene_controller = SceneController::new(
        WindowBuilder::new()
            .with_inner_size(Size::Logical(LogicalSize::new(
                initial_width,
                initial_height,
            )))
            .build(&event_loop)
            .unwrap(),
    );
    scene_controller.create_layer(LayerCreationCommand::Sample(
        layer_model::sample::SampleLayerCreationCommand(Rect::new(
            Point::new(0.0, 0.0),
            // TODO: 400 에서 줄이면 에러:
            // copy would end up overruning the bounds of one of the buffers or textures
            layer_model::Size::new(400.0, 400.0),
        )),
    ));
    scene_controller.update();
    scene_controller.render();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == scene_controller.window.id() => {
            // TODO: 원래는 이런 코드가 있었음. 카메라 컨트롤 관련인데 아마도 learn wgpu 문서에 나와있을듯
            // if !quad_renderer.input(event) {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                WindowEvent::CursorMoved { position, .. } => {
                    log::debug!("cursor moved: {}, {}", position.x, position.y);
                }
                WindowEvent::Resized(physical_size) => {
                    scene_controller.handle_resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    scene_controller.handle_resize(**new_inner_size);
                }
                _ => {}
            }
        }
        Event::RedrawRequested(_) => {
            scene_controller.update();
            scene_controller.render();
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            scene_controller.window.request_redraw();
        }
        _ => {}
    });
}

pub struct SceneController {
    pub window: Window,
    quad_renderer: QuadRenderer,
}

impl SceneController {
    fn new(window: Window) -> Self {
        let quad_renderer = block_on(QuadRenderer::new(&window));
        SceneController {
            window,
            quad_renderer,
        }
    }

    fn create_layer(&mut self, command: LayerCreationCommand) -> LayerId {
        let i = 1;
        let quad_handle =
            self.quad_renderer
                .new_quad((i as f32) * 30.0, (i as f32) * 30.0, 100.0, 100.0);
        let sample_layer = SampleLayerProps {
            content_rect: Rect::new(
                Point::new(0.0, 0.0),
                // TODO: 400 에서 줄이면 에러:
                // copy would end up overruning the bounds of one of the buffers or textures
                layer_model::Size::new(400.0, 400.0),
            ),
            opacity: 1.0,
            border: None,
            fill: None,
        };
        let image = layer_renderer::paint_sample_layer(&sample_layer);
        self.quad_renderer
            .update_texture(quad_handle, image.as_slice());
        // FIXME: mock layer id
        "mock_id".into()
    }

    fn update(&mut self) {
        self.quad_renderer.update();
    }

    fn render(&mut self) {
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
