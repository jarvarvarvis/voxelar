#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (set = 0, binding = 0) uniform camera_buffer
{
    mat4 projection_matrix;
    mat4 view_matrix;
} CameraBuffer;

const vec2 VERTEX_COORDS[6] = vec2[](
    vec2(-1.0, -1.0),
    vec2(-1.0,  1.0),
    vec2( 1.0, -1.0),
    vec2( 1.0, -1.0),
    vec2(-1.0,  1.0),
    vec2( 1.0,  1.0)
);

void main() {
    vec3 pos = vec3(VERTEX_COORDS[gl_VertexIndex], 0.0);
    mat4 vp_matrix = CameraBuffer.projection_matrix * CameraBuffer.view_matrix;
    gl_Position = vp_matrix * vec4(pos, 1.0);
}
