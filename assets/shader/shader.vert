#version 460

layout(location = 0) out vec3 frag_color;
layout(location = 1) out vec2 frag_texture_coords;

layout(location = 0) in vec2 texture_coords;

vec2[4] vertices = {
    {-1.0, -1.0},
    {1.0, -1.0},
    {-1.0, 1.0},
    {1.0, 1.0},
};

layout(set = 0, binding = 0) uniform UniformGlobalObject {
    float ratio;
    float scale;
} ugo;

layout(set = 0, binding = 1) uniform UniformInstanceObject {
    float x;
    float y;
} uio;

void main() {
    vec2 pos = ugo.scale * vec2(vertices[gl_VertexIndex].x * ugo.ratio, vertices[gl_VertexIndex].y);
    gl_Position = vec4(pos.x + uio.x, pos.y + uio.y, 0.0, 1.0);
    frag_color = vec3(1.0);
    frag_texture_coords = texture_coords;
}
