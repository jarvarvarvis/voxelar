#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec2 vertex_uv;

layout (location = 0) out vec4 frag_color;

layout (set = 0, binding = 1) uniform sampler2D input_texture;

void main() {
    vec3 color = texture(input_texture, vertex_uv).xyz;
    frag_color = vec4(color, 1.0);
}
