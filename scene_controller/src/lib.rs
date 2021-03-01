use futures::executor::block_on;
use layer_model::commands::LayerCreationCommand;
use layer_model::rect::RectProps;
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

pub fn main() {
    let initial_width = 400.;
    let initial_height = 400.;
    // TODO: 400 에서 줄이면 에러:
    // copy would end up overruning the bounds of one of the buffers or textures

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
    let root_layer_id = *scene_controller.layer_repository.root_layer_id();
    scene_controller.layer_repository.create_sample_layer(
        &root_layer_id,
        &Rect::new(Point::new(0.0, 0.0), layer_model::Size::new(100.0, 100.0)),
    );
    scene_controller.layer_repository.create_sample_layer(
        &root_layer_id,
        &Rect::new(Point::new(50.0, 50.0), layer_model::Size::new(200.0, 200.0)),
    );
    scene_controller.layer_repository.create_sample_layer(
        &root_layer_id,
        &Rect::new(
            Point::new(100.0, 100.0),
            layer_model::Size::new(300.0, 300.0),
        ),
    );
    scene_controller.layer_repository.create_rect_layer(
        &root_layer_id,
        RectProps {
            content_rect: Rect::new(Point::new(0.0, 200.0), layer_model::Size::new(100.0, 100.0)),
            opacity: 1.0,
            fill: Some(Fill::Color {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            }),
            border: Some(Border {
                fill: Fill::Color {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255,
                },
                width: 10.0,
                position: BorderPosition::Inner,
            }),
        },
    );

    let container_id = scene_controller.layer_repository.create_layer(
        &root_layer_id,
        Layer::Container(ContainerProps {
            content_rect: Rect::new(
                Point::new(200.0, 200.0),
                layer_model::Size::new(100.0, 100.0),
            ),
            opacity: 0.5,
            fill: Some(Fill::Color {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            }),
            border: Some(Border {
                fill: Fill::Color {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255,
                },
                width: 10.0,
                position: BorderPosition::Inner,
            }),
            children: vec![],
        }),
    );

    scene_controller.layer_repository.create_rect_layer(
        &container_id,
        RectProps {
            content_rect: Rect::new(
                Point::new(100.0, 100.0),
                layer_model::Size::new(100.0, 100.0),
            ),
            opacity: 1.0,
            fill: Some(Fill::Color {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            }),
            border: Some(Border {
                fill: Fill::Color {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255,
                },
                width: 10.0,
                position: BorderPosition::Inner,
            }),
        },
    );

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
    layer_repository: LayerRepository,
    // TODO: 지금은 텍스처 하나에 전부 다 그리지만, 개선되어야 함
    root_quad_id: QuadId,
    root_draw_target: raqote::DrawTarget,
}

impl SceneController {
    fn new(window: Window) -> Self {
        let width = window.inner_size().width as f32;
        let height = window.inner_size().height as f32;
        let mut quad_renderer = block_on(QuadRenderer::new(&window));
        let root_quad_id = quad_renderer.new_quad(-200.0, -200.0, width, height);
        let layer_repository = LayerRepository::new(layer_model::common::Size::new(width, height));
        let root_draw_target = raqote::DrawTarget::new(width as i32, height as i32);
        SceneController {
            window,
            quad_renderer,
            root_quad_id,
            root_draw_target,
            layer_repository,
        }
    }

    fn update(&mut self) {
        self.quad_renderer.update();
    }

    fn render(&mut self) {
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
