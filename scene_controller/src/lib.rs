use futures::executor::block_on;
use layer_model::*;
use wgpu_renderer::wgpu_layer::*;
use winit::window::Window;
mod scheduler;

pub fn sample_draw(window: &Window) {}

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
    let window = WindowBuilder::new()
        .with_inner_size(Size::Logical(LogicalSize::new(
            initial_width,
            initial_height,
        )))
        .build(&event_loop)
        .unwrap();
    let mut quad_renderer = block_on(QuadRenderer::new(&window));
    let mut quad_handles = Vec::new();
    for i in 0..3 {
        // 여기서 quad 를 직접 생성하는 것이 아니라,
        // layer -> renderer -> layer_compositor -> quad 로 거쳐와야 함
        let quad_handle =
            quad_renderer.new_quad((i as f32) * 30.0, (i as f32) * 30.0, 100.0, 100.0);
        quad_handles.push(quad_handle);
        let image = layer_renderer::default_image(Rect::new(
            Point::new(0.0, 0.0),
            // TODO: 400 에서 줄이면 에러:
            // copy would end up overruning the bounds of one of the buffers or textures
            layer_model::Size::new(400.0, 400.0),
        ));
        quad_renderer.update_texture(quad_handle, image.as_slice());
    }

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !quad_renderer.input(event) {
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
                        quad_renderer.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        quad_renderer.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(_) => {
            quad_renderer.update();
            match quad_renderer.render() {
                Ok(_) => {}
                Err(RendererError::SwapChainLost) => quad_renderer.resize(quad_renderer.size),
                Err(RendererError::SwapChainOutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {}
    });
}
