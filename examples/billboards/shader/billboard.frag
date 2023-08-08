#version 460

#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) in vec2 frag_offset;

layout (location = 0) out vec4 frag_color;

void main() {
    frag_color = vec4(1.0);
}
