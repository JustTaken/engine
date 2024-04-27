#version 460

layout(location = 0) out vec3 frag_color;
layout(location = 0) in vec2 positions;

/* layout(set = 0, binding = 0) uniform UniformGlobalObject { */
/*   mat4 view; */
/*   mat4 proj; */
/* } ugo; */

void main() {
  /* gl_Position = ugo.proj * ugo.view * vec4(positions, 1.0); */
  gl_Position = vec4(positions, 0.0, 1.0);
  frag_color = vec3(1.0);
}
