use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use layer_model::rect::RectProps;
use layer_model::*;
use raqote::{
    DrawOptions, DrawTarget, Path, PathBuilder, SolidSource, Source, StrokeStyle, Transform,
};

pub fn render_scene(layer_repo: &LayerRepository, draw_target: &mut DrawTarget) {
    render_container(draw_target, layer_repo.root_container_layer(), layer_repo);
}

pub fn render_layer(layer: &Layer, layer_repo: &LayerRepository, draw_target: &mut DrawTarget) {
    use Layer::*;
    match layer {
        Container(ref props) => render_container(draw_target, props, layer_repo),
        Rect(ref props) => render_rect(draw_target, props),
        Sample(ref props) => paint_sample_layer(draw_target, props),
    }
}

pub fn render_container(
    draw_target: &mut DrawTarget,
    props: &ContainerProps,
    layer_repo: &LayerRepository,
) {
    if !props.is_opaque() {
        draw_target.push_layer(props.opacity);
    }
    if props.fill.is_some() || props.border.is_some() {
        paint_container(draw_target, props);
    }

    {
        let prev_transform = *draw_target.get_transform();
        let translation =
            Transform::create_translation(props.content_rect.origin.x, props.content_rect.origin.y);
        let next_transform = prev_transform.post_transform(&translation);
        draw_target.set_transform(&next_transform);
        for child_id in &props.children {
            let child_layer = layer_repo.get_layer_by_id(child_id);
            render_layer(child_layer, layer_repo, draw_target);
        }
        test_text(draw_target);
        draw_target.set_transform(&prev_transform);
    }
    if !props.is_opaque() {
        draw_target.pop_layer();
    }
}

fn test_text(draw_target: &mut DrawTarget) {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    draw_target.fill_rect(
        0.0,
        0.0,
        10.0,
        10.0,
        &Source::Solid(SolidSource {
            r: 0xff,
            g: 0xff,
            b: 0,
            a: 0,
        }),
        &DrawOptions::new(),
    );

    draw_target.draw_text(
        &font,
        24.,
        "Hello",
        Point::new(0., 100.),
        &Source::Solid(SolidSource {
            r: 0xff,
            g: 0,
            b: 0xff,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );
}

pub fn render_rect(draw_target: &mut DrawTarget, props: &RectProps) {
    // TODO: cache invalidation logic
    paint_rect(draw_target, props);
}

fn paint_fill(draw_target: &mut DrawTarget, fill: &Fill, path: &Path) {
    let draw_option = DrawOptions::new();
    let source = match fill {
        Fill::Color { r, g, b, a } => Source::Solid(SolidSource {
            r: *r,
            g: *g,
            b: *b,
            a: *a,
        }),
    };
    draw_target.fill(path, &source, &draw_option);
}

fn paint_border(draw_target: &mut DrawTarget, border: &Border, path: &Path) {
    let draw_option = DrawOptions::new();
    let source = match &border.fill {
        Fill::Color { r, g, b, a } => Source::Solid(SolidSource {
            r: *r,
            g: *g,
            b: *b,
            a: *a,
        }),
    };
    // TODO: border position
    let stroke_style = StrokeStyle {
        width: border.width,
        ..Default::default()
    };
    draw_target.stroke(&path, &source, &stroke_style, &draw_option);
}

fn paint_rect(draw_target: &mut DrawTarget, props: &RectProps) {
    if !props.is_opaque() {
        draw_target.push_layer(props.opacity);
    }
    let mut pb = PathBuilder::new();
    // TODO: Trait-bounded generic paint function to share
    let origin = props.content_rect.origin;
    let size = props.content_rect.size;
    pb.rect(origin.x, origin.y, size.width, size.height);
    let path = pb.finish();

    if let Some(ref fill) = props.fill {
        paint_fill(draw_target, fill, &path);
    }
    if let Some(ref border) = props.border {
        paint_border(draw_target, border, &path);
    }
    if !props.is_opaque() {
        draw_target.pop_layer();
    }
}

fn paint_container(draw_target: &mut DrawTarget, props: &ContainerProps) {
    let mut pb = PathBuilder::new();
    let origin = props.content_rect.origin;
    let size = props.content_rect.size;
    pb.rect(origin.x, origin.y, size.width, size.height);
    let path = pb.finish();
    if let Some(ref fill) = props.fill {
        paint_fill(draw_target, fill, &path);
    }
    if let Some(ref border) = props.border {
        paint_border(draw_target, border, &path);
    }
}

pub fn paint_sample_layer(dt: &mut DrawTarget, l: &SampleLayerProps) {
    default_image(dt, l.content_rect())
}

fn default_image(dt: &mut DrawTarget, rect: Rect) {
    let horizontal_ratio = rect.size.width / 400.0;
    let vertical_ratio = rect.size.height / 400.0;
    let prev_transform = *dt.get_transform();
    let translation = Transform::create_translation(rect.origin.x, rect.origin.y);
    dt.set_transform(
        &prev_transform
            .post_scale(horizontal_ratio, vertical_ratio)
            .post_transform(&translation),
    );
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
    dt.set_transform(&prev_transform);
}
