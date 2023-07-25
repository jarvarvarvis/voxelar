#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#include "intersect/box.glsl"
#include "intersect/quadric_proj.glsl"

layout (location = 0) in vec3 pos;

layout (set = 0, binding = 0) uniform camera_buffer
{
    mat4 mvp_matrix;
    vec2 screen_size;
} CameraBuffer;

layout (location = 0) out Box current_box;

const float VOXEL_SIZE = 2.0;

void main() {
    gl_Position = CameraBuffer.mvp_matrix * vec4(pos, 1.0);
    gl_PointSize = VOXEL_SIZE;
}
