#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

#include "intersect/box.glsl"
#include "intersect/quadric_proj.glsl"

layout (location = 0) in vec3 local_vertex_position;

layout (set = 0, binding = 0) uniform camera_buffer
{
    vec4 camera_position;
    mat4 mvp_matrix;
    mat4 view_matrix;
    vec2 screen_size;
    float z_far;
} CameraBuffer;

struct RaytraceInput {
    Box current_box;
};

layout (location = 0) out RaytraceInput raytrace_input;

const float VOXEL_SIZE = 5.0;

void main() {
    vec4 clip_space_position = CameraBuffer.mvp_matrix * vec4(local_vertex_position, 1.0);
    float point_size;

    quadricProj(
        local_vertex_position,
        VOXEL_SIZE,
        CameraBuffer.mvp_matrix,
        CameraBuffer.screen_size,
        clip_space_position,
        point_size
    );

    // Square area
    float stochastic_coverage = point_size * point_size;
    if ((stochastic_coverage < 0.8) && ((gl_VertexIndex & 0xffff) > stochastic_coverage * (0xffff / 0.8))) {
        // "Cull" small voxels in a stable, stochastic way by moving past the z = 0 plane.
        // Assumes voxels are in randomized order.
        clip_space_position = vec4(-1,-1,-1,-1);
    }

    gl_Position = clip_space_position;
    gl_PointSize = point_size;

    Box box;
    box.radius = vec3(VOXEL_SIZE / 2.0);
    box.center = local_vertex_position * VOXEL_SIZE;
    box.invRadius = safeInverse(box.radius);
    box.rotation = mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);

    raytrace_input.current_box = box;
}
