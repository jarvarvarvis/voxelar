#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 vertex_color;
layout (location = 0) out vec4 uFragColor;

layout (set = 0, binding = 1) uniform scene_buffer
{
    vec4 ambient_color;
} SceneBuffer;

void main() {
    uFragColor = vec4(SceneBuffer.ambient_color.xyz * vertex_color, 1.0);
}
