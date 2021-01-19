mod wgpu_layer;

use futures::executor::block_on;
use raqote::*;
use wgpu::util::DeviceExt;
use wgpu_layer::*;
use winit::{
    dpi::{LogicalSize, Size},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
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
        let quad_handle =
            quad_renderer.new_quad((i as f32) * 30.0, (i as f32) * 30.0, 100.0, 100.0);
        quad_handles.push(quad_handle);
        let image = default_image();
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
                Err(wgpu::SwapChainError::Lost) => quad_renderer.resize(quad_renderer.size),
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
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

fn default_image() -> Vec<u8> {
    let mut dt = DrawTarget::new(400, 400);
    let mut pb = PathBuilder::new();
    pb.move_to(100., 10.);
    pb.cubic_to(150., 40., 175., 0., 200., 10.);
    pb.quad_to(120., 100., 80., 200.);
    pb.quad_to(150., 180., 300., 300.);
    pb.close();
    let path = pb.finish();
    let gradient = Source::new_radial_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.2,
                    color: Color::new(0xff, 0, 0xff, 0),
                },
                GradientStop {
                    position: 0.8,
                    color: Color::new(0xff, 0xff, 0xff, 0xff),
                },
                GradientStop {
                    position: 1.,
                    color: Color::new(0xff, 0xff, 0, 0xff),
                },
            ],
        },
        Point::new(150., 150.),
        128.,
        Spread::Pad,
    );
    dt.fill(&path, &gradient, &DrawOptions::new());
    let mut pb = PathBuilder::new();
    pb.move_to(100., 100.);
    pb.line_to(300., 300.);
    pb.line_to(200., 300.);
    let path = pb.finish();
    dt.stroke(
        &path,
        &Source::Solid(SolidSource {
            r: 0x0,
            g: 0x0,
            b: 0x80,
            a: 0x80,
        }),
        &StrokeStyle {
            cap: LineCap::Round,
            join: LineJoin::Round,
            width: 10.,
            miter_limit: 2.,
            dash_array: vec![10., 18.],
            dash_offset: 16.,
        },
        &DrawOptions::new(),
    );
    dt.get_data_u8().to_vec()
}
