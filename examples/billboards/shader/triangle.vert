#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

layout (location = 0) out vec2 vertex_uv;

layout (set = 0, binding = 0) uniform camera_buffer
{
    mat4 projection_matrix;
    mat4 view_matrix;
} CameraBuffer;

void main() {
    mat4 vp_matrix = CameraBuffer.projection_matrix * CameraBuffer.view_matrix;
    gl_Position = vp_matrix * vec4(pos, 1.0);

    vertex_uv = uv;
}
