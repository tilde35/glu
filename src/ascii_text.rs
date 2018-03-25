use ascii_text_img::ascii_raw_img;
use glium;
use glium::backend::Facade;

#[derive(Clone, Copy, Debug)]
struct AsciiVertex {
    position: [f32; 2],
    color: [f32; 4],
    texcoord: [f32; 2],
}
implement_vertex!(AsciiVertex, position, color, texcoord);

pub struct AsciiText {
    ascii_texture: glium::texture::Texture2d,
    ascii_program: glium::Program,
}
impl AsciiText {
    pub fn new(display: &glium::Display) -> Self {
        let raw = ascii_raw_img();

        let program_src = include_str!("ascii_text.glsl").to_string();
        let program = glium::Program::from_source(
            display,
            &program_src.replace("TEMPLATE_PROGRAM", "VERTEX_PROGRAM"),
            &program_src.replace("TEMPLATE_PROGRAM", "FRAGMENT_PROGRAM"),
            None,
        ).expect("Failed to compile ASCII shader: ascii_text.glsl");

        Self {
            ascii_texture: glium::texture::Texture2d::new(display, raw).expect("Failed to load ASCII texture"),
            ascii_program: program,
        }
    }

    pub fn draw_white<DrawSurface: glium::Surface>(&self, display: &glium::Display, target: &mut DrawSurface, txt: &str, scale: f32, pos: [f32; 2]) {
        self.draw(display, target, txt, scale, pos, [1.0, 1.0, 1.0, 1.0])
    }
    pub fn draw_black<DrawSurface: glium::Surface>(&self, display: &glium::Display, target: &mut DrawSurface, txt: &str, scale: f32, pos: [f32; 2]) {
        self.draw(display, target, txt, scale, pos, [0.0, 0.0, 0.0, 1.0])
    }

    pub fn draw<DrawSurface: glium::Surface>(&self, display: &glium::Display, target: &mut DrawSurface, txt: &str, scale: f32, pos: [f32; 2], color: [f32; 4]) {
        let win_size = display.get_context().get_framebuffer_dimensions();

        let transform = {
            // Scale and translate values
            let (w, h) = (win_size.0 as f32, win_size.1 as f32);
            let xs: f32 = 2.0 / w;
            let xt: f32 = -w / 2.0;
            let ys: f32 = -2.0 / h;
            let yt: f32 = -h / 2.0;

            [
                [xs, 0.0, 0.0, xt * xs],
                [0.0, ys, 0.0, yt * ys],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };

        let uniforms = uniform! {
            matrix: transform,
            ascii_texture: glium::uniforms::Sampler::new(&self.ascii_texture)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest),
        };

        let mut draw_params: glium::draw_parameters::DrawParameters = Default::default();
        draw_params.blend = glium::Blend::alpha_blending();

        let mut vertex_data = Vec::with_capacity(4 * txt.len());
        let mut index_data = Vec::with_capacity(6 * txt.len());

        let dim = scale * 8.0;
        let next_char = scale * 9.0;
        let mut x = pos[0];
        let mut y = pos[1];
        for glyph in txt.as_bytes() {
            if *glyph == b'\n' {
                // Newline
                x = pos[0];
                y += next_char;
            } else {
                add_glyph(
                    &mut vertex_data,
                    &mut index_data,
                    *glyph,
                    dim,
                    [x, y],
                    color,
                );
                x += next_char;
            }
        }

        let vertex_buffer = glium::VertexBuffer::immutable(display, &vertex_data).expect("Failed to create ASCII vertex buffer");
        let indices = glium::index::IndexBuffer::immutable(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &index_data,
        ).expect("Failed to create ASCII index buffer");

        target
            .draw(
                &vertex_buffer,
                &indices,
                &self.ascii_program,
                &uniforms,
                &draw_params,
            )
            .expect("Failed to render ASCII text");
    }
}

fn add_glyph(vertex_data: &mut Vec<AsciiVertex>, index_data: &mut Vec<u32>, glyph: u8, dim: f32, pos: [f32; 2], color: [f32; 4]) {
    let idx = vertex_data.len() as u32;

    let x = pos[0];
    let y = pos[1];

    let tx = glyph % 16;
    let ty = glyph / 16;

    let tx = 8.0 * (tx as u32 as f32) / 128.0;
    let ty = 8.0 * (ty as u32 as f32) / 128.0;

    let tdim = 8.0 / 128.0;

    vertex_data.push(AsciiVertex {
        position: [x, y],
        texcoord: [tx, ty],
        color,
    });
    vertex_data.push(AsciiVertex {
        position: [x + dim, y],
        texcoord: [tx + tdim, ty],
        color,
    });
    vertex_data.push(AsciiVertex {
        position: [x + dim, y + dim],
        texcoord: [tx + tdim, ty + tdim],
        color,
    });
    vertex_data.push(AsciiVertex {
        position: [x, y + dim],
        texcoord: [tx, ty + tdim],
        color,
    });

    index_data.push(idx + 0);
    index_data.push(idx + 1);
    index_data.push(idx + 3);

    index_data.push(idx + 1);
    index_data.push(idx + 2);
    index_data.push(idx + 3);
}
