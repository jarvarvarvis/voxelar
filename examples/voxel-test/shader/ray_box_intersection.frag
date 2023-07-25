#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#include "intersect/box.glsl"

layout (location = 0) in Box current_box;

layout (location = 0) out vec4 frag_color;

void main() {
    frag_color = vec4(1.0);
}
