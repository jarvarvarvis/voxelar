#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#include "intersect/intersect.glsl"

struct RaytraceInput {
    Box current_box;
};

layout (location = 0) in RaytraceInput raytrace_input;

layout (set = 0, binding = 0) uniform camera_buffer
{
    vec4 camera_position;
    mat4 mvp_matrix;
    mat4 view_matrix;
    vec2 screen_size;
    float z_far;
} CameraBuffer;

layout (location = 0) out vec4 frag_color;

void main() {
    vec2 uv = gl_FragCoord.xy / CameraBuffer.screen_size;

    Ray ray;
    ray.origin = CameraBuffer.camera_position.xyz;

    vec2 rescaled_uv = uv * 2.0 - 1.0;
    vec4 view_rot = transpose(CameraBuffer.view_matrix) * vec4(-rescaled_uv, 1.0, 1.0);
    ray.direction = view_rot.xyz;

    float distance;
    vec3 normal;
    if (ourIntersectBox(raytrace_input.current_box, ray, distance, normal, true, vec3(0.0))) {
        frag_color = vec4((normal + 1.0) / 2.0, 1.0);
        gl_FragDepth = distance / CameraBuffer.z_far; // Adjust depth
    } else {
        frag_color = vec4(1.0);
        gl_FragDepth = 1.0;
    }
}
