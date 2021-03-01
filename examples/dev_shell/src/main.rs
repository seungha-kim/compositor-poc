use layer_model::rect::RectProps;
use layer_model::*;
use scene_controller::*;
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};
use winit::event::{Event, KeyboardInput};
use winit::event_loop::{ControlFlow, EventLoop};

struct Layers {
    rect_layer_id: LayerId,
}

struct SampleLogic {
    layers: Option<Layers>,
}

impl SceneLogic for SampleLogic {
    fn initialize_scene(&mut self, layer_repo: &mut LayerRepository) {
        let root_layer_id = *layer_repo.root_layer_id();
        let rect_layer_id = layer_repo.create_sample_layer(
            &root_layer_id,
            &Rect::new(Point::new(0.0, 0.0), layer_model::Size::new(100.0, 100.0)),
        );
        layer_repo.create_sample_layer(
            &root_layer_id,
            &Rect::new(Point::new(50.0, 50.0), layer_model::Size::new(200.0, 200.0)),
        );
        layer_repo.create_sample_layer(
            &root_layer_id,
            &Rect::new(
                Point::new(100.0, 100.0),
                layer_model::Size::new(300.0, 300.0),
            ),
        );
        layer_repo.create_rect_layer(
            &root_layer_id,
            RectProps {
                content_rect: Rect::new(
                    Point::new(0.0, 200.0),
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

        let container_id = layer_repo.create_layer(
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

        layer_repo.create_rect_layer(
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

        self.layers = Some(Layers { rect_layer_id });
    }

    fn handle_window_event(
        &mut self,
        window_event: &WindowEvent,
        layer_repo: &mut LayerRepository,
    ) -> ControlFlow {
        match window_event {
            WindowEvent::CloseRequested => ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                } => ControlFlow::Exit,
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(keycode),
                    ..
                } => {
                    let rect_layer_id = self.layers.as_ref().unwrap().rect_layer_id;
                    let layer = layer_repo.get_layer_by_id_mut(&rect_layer_id);
                    if let Layer::Sample(props) = layer {
                        match keycode {
                            VirtualKeyCode::Right => {
                                props.content_rect.origin.x += 1.0;
                            }
                            VirtualKeyCode::Left => {
                                props.content_rect.origin.x -= 1.0;
                            }
                            VirtualKeyCode::Up => {
                                props.content_rect.origin.y -= 1.0;
                            }
                            VirtualKeyCode::Down => {
                                props.content_rect.origin.y += 1.0;
                            }
                            _ => {}
                        }
                    }
                    ControlFlow::Poll
                }
                _ => ControlFlow::Poll,
            },
            WindowEvent::CursorMoved { position, .. } => {
                log::debug!("cursor moved: {}, {}", position.x, position.y);
                ControlFlow::Poll
            }
            WindowEvent::Resized(physical_size) => {
                // TODO: resize
                // scene_controller.handle_resize(*physical_size);
                ControlFlow::Poll
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // TODO: resize
                // scene_controller.handle_resize(**new_inner_size);
                ControlFlow::Poll
            }
            _ => ControlFlow::Poll,
        }
    }

    fn update(&mut self, layer_repo: &mut LayerRepository) {}
}

pub fn main() {
    // TODO: 윈도우 사이즈는 u32가 되어야 함
    let initial_width = 400.;
    let initial_height = 400.;
    let event_loop = EventLoop::new();
    env_logger::init();

    let mut logic = SampleLogic { layers: None };

    let mut scene_controller =
        SceneController::new(&event_loop, initial_width, initial_height, logic);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == scene_controller.window.id() => {
            // TODO: 원래는 이런 코드가 있었음. 카메라 컨트롤 관련인데 아마도 learn wgpu 문서에 나와있을듯
            // if !quad_renderer.input(event) {
            *control_flow = scene_controller
                .logic
                .handle_window_event(event, &mut scene_controller.layer_repository);
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
