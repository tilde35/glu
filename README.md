# glu - Small Simple GL Utilities #

The glu project is a set of small, simple OpenGL utilities for use in my personal projects.

Cargo Dependency: `glu = { git = "https://github.com/tilde35/glu" }`

Legacy Cargo Dependency (with tuple coordinates): `glu = { git = "https://github.com/tilde35/glu", branch = "tuple-coords" }`

For a more complete setup, refer to the  [example project](examples/fps_time_step.rs).

### Utility List ###
* ASCII Text
* Events and Event State
* Time Step

## ASCII Text ##

```rust
use glu::AsciiText;

// Intialization
let ascii_text = AsciiText::new(&display);

// Render text (text, scale, location, color)
ascii_text.draw(&display, &mut target, "Example Text", 2.0, [15.0, 15.0], [0.8, 0.9, 0.8, 1.0]);

// Render white/black text (text, scale, location)
ascii_text.draw_white(&display, &mut target, "Example Text", 2.0, [15.0, 15.0]);
ascii_text.draw_black(&display, &mut target, "Example Text", 2.0, [15.0, 15.0]);
```

## Events and Event State ##

For more information, refer to the [event quick-reference guide](Events.md).

```rust
use glu::{Event, EventState};

// Intialization
let mut event_state = EventState::new();

// Event loop
events_loop.poll_events(|event| {
    let e = Event::from_gl(&event, &mut event_state);
    match e {
        Event::WindowClose(..) => exit = true,
        _ => {}
    }
});
```

## Time Step ##

```rust
use glu::TimeStep;

// Intialization
let mut step = TimeStep::for_freq_ms(250);

// Per frame
step.tick(|| simulator.next_step());
```

# External Library Quick-Reference #

## fps_counter  ##

Cargo Dependency: `fps_counter = "1.0.0"`

### Usage ###

```rust
use fps_counter::FPSCounter;

let mut fps = FPSCounter::new();

loop {
  let cur_fps = fps.tick();
  // Process events and render page...
}
```

## cgmath ##

Cargo Dependency: `cgmath = "0.16.1"`

### Usage ###

(Placeholder for view matrix usage)


# Quick Setup Guides #

## Shaders ##

Shader Definition

```glsl
#version 150
#define TEMPLATE_PROGRAM

uniform mat4 matrix;
uniform sampler2D image_tx;

#ifdef VERTEX_PROGRAM

in vec3 position;
in vec2 tx_coords;

out vec2 v_tx_coords;

void main() {
    v_tx_coords = tx_coords;
    gl_Position = matrix * vec4(position, 1.0);
}

#else

in vec2 v_tx_coords;

out vec4 color;

void main() {
    color = texture(image_tx, v_tx_coords);
}

#endif
```

Loading the Shader

```rust
let example_program = {
    let program_src = include_str!("example.glsl").to_string();
    glium::Program::from_source(
        &display,
        &program_src.replace("TEMPLATE_PROGRAM", "VERTEX_PROGRAM"),
        &program_src.replace("TEMPLATE_PROGRAM", "FRAGMENT_PROGRAM"),
        None,
    ).expect("Failed to load example.glsl")
};
```

## Basic Rendering ##

Vertex Data Definition

```rust
#[derive(Clone, Copy, Debug)]
pub struct VertexData {
    pub pos: [f32; 3],
    pub txcoord: [f32; 2],
}
implement_vertex!(VertexData, pos, txcoord);
```

Texture Loading

(Texture loading placeholder)

Vertex/Index Buffers

```rust
// This example generates the data for a list of quad vertexes.
let vertex_list: Vec<VertexData> = ...; // Quad coordinates in clockwise order
let quad_vertex_buffer = glium::VertexBuffer::new(&display, &vertex_list).unwrap();

// 6 index entries for every quad (4 vertex entries)
let len = (triangle_list.len() * 3) / 2;
let mut index_data = Vec::with_capacity(len);
let mut v_idx = 0;
while v_idx < len {
    index_data.push(v_idx as u32 + 0);
    index_data.push(v_idx as u32 + 1);
    index_data.push(v_idx as u32 + 2);

    index_data.push(v_idx as u32 + 0);
    index_data.push(v_idx as u32 + 2);
    index_data.push(v_idx as u32 + 3);

    v_idx += 4;
}

let quad_index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &index_data).unwrap();
```

Rendering

```rust
let mut draw_params: glium::draw_parameters::DrawParameters = Default::default();
draw_params.depth = glium::Depth {
    test: glium::draw_parameters::DepthTest::IfLessOrEqual,
    write: true,
    ..Default::default()
};
draw_params.backface_culling = glium::BackfaceCullingMode::CullCounterClockwise;
let uniforms = uniform! {
    matrix: Into::<[[f32; 4]; 4]>::into(view_matrix),
    image_tx: glium::uniforms::Sampler::new(&image_tx)
        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest),
};
target
    .draw(&vertex_buffer, &index_buffer, &example_program, &uniforms, &draw_params)
    .unwrap();
```
