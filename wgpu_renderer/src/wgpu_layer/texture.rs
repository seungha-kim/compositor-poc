use anyhow::*;
use raqote::*;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn test(device: &wgpu::Device, queue: &wgpu::Queue, label: Option<&str>) -> Result<Self> {
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
        let data = dt.get_data_u8();

        let size = wgpu::Extent3d {
            width: 400,
            height: 400,
            depth: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1, // mipmap?
            sample_count: 1,    // MSAA
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            data,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * 400,
                rows_per_image: 400,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
