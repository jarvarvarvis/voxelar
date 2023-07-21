#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

layout (location = 0) out vec2 vertex_uv;

layout (set = 0, binding = 0) uniform camera_buffer
{
    mat4 mvp_matrix;
} CameraBuffer;

void main() {
    gl_Position = CameraBuffer.mvp_matrix * vec4(pos, 1.0);

    vertex_uv = uv;
}
