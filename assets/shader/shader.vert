#version 460

layout(location = 0) out vec3 frag_color;
layout(location = 1) out vec2 frag_texture_coords;

layout(location = 0) in vec2 positions;
layout(location = 1) in vec2 texture_coords;

layout(set = 0, binding = 0) uniform UniformGlobalObject {
    mat4 view;
    mat4 model;
} ugo;

void main() {
  gl_Position = ugo.view * ugo.model * vec4(positions, 0.0, 1.0);
  frag_color = vec3(1.0);
  frag_texture_coords = texture_coords;
}
