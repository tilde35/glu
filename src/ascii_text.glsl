#version 140
#define TEMPLATE_PROGRAM

uniform mat4 matrix;
uniform sampler2D ascii_texture;

#ifdef VERTEX_PROGRAM

in vec2 position;
in vec4 color;
in vec2 texcoord;

smooth out vec4 v_color;
smooth out vec2 v_texcoord;

void main() {
  v_color = color;
  v_texcoord = texcoord;
  gl_Position = vec4(position, 0.0, 1.0) * matrix;
}

#else

smooth in vec4 v_color;
smooth in vec2 v_texcoord;

out vec4 f_color;

void main() {
  float a = texture(ascii_texture, v_texcoord).a;
  if (a < 0.01) {
    discard;
  }
  else {
    f_color = v_color;
  }
}

#endif
