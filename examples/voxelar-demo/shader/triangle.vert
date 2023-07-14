#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 pos;

layout (location = 0) out vec4 o_color;

layout (set = 0, binding = 0) uniform camera_buffer
{
    mat4 mvp_matrix;
} CameraBuffer;

void main() {
    gl_Position = CameraBuffer.mvp_matrix * vec4(pos, 1.0);
    o_color = vec4(1.0 - (pos + vec3(1.0)) / 2.0, 1.0);
}
