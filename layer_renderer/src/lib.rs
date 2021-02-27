use layer_model::*;

pub fn paint_sample_layer(l: &SampleLayerProps) -> Vec<u8> {
    default_image(l.content_rect())
}

fn default_image(rect: Rect) -> Vec<u8> {
    let mut dt = raqote::DrawTarget::new(rect.size.width as i32, rect.size.height as i32);
    // let mut dt = raqote::DrawTarget::new(400, 400);
    let mut pb = raqote::PathBuilder::new();
    pb.move_to(100., 10.);
    pb.cubic_to(150., 40., 175., 0., 200., 10.);
    pb.quad_to(120., 100., 80., 200.);
    pb.quad_to(150., 180., 300., 300.);
    pb.close();
    let path = pb.finish();
    let gradient = raqote::Source::new_radial_gradient(
        raqote::Gradient {
            stops: vec![
                raqote::GradientStop {
                    position: 0.2,
                    color: raqote::Color::new(0xff, 0, 0xff, 0),
                },
                raqote::GradientStop {
                    position: 0.8,
                    color: raqote::Color::new(0xff, 0xff, 0xff, 0xff),
                },
                raqote::GradientStop {
                    position: 1.,
                    color: raqote::Color::new(0xff, 0xff, 0, 0xff),
                },
            ],
        },
        Point::new(150., 150.),
        128.,
        raqote::Spread::Pad,
    );
    dt.fill(&path, &gradient, &raqote::DrawOptions::new());
    let mut pb = raqote::PathBuilder::new();
    pb.move_to(100., 100.);
    pb.line_to(300., 300.);
    pb.line_to(200., 300.);
    let path = pb.finish();
    dt.stroke(
        &path,
        &raqote::Source::Solid(raqote::SolidSource {
            r: 0x0,
            g: 0x0,
            b: 0x80,
            a: 0x80,
        }),
        &raqote::StrokeStyle {
            cap: raqote::LineCap::Round,
            join: raqote::LineJoin::Round,
            width: 10.,
            miter_limit: 2.,
            dash_array: vec![10., 18.],
            dash_offset: 16.,
        },
        &raqote::DrawOptions::new(),
    );
    dt.get_data_u8().to_vec()
}
