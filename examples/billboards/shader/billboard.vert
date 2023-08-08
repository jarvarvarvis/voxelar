#version 460

#extension GL_ARB_separate_shader_objects : enable

layout (set = 0, binding = 0) uniform camera_buffer
{
    mat4 projection_matrix;
    mat4 view_matrix;
} CameraBuffer;

struct BillboardBufferData {
    vec4 position;
};

layout (std430, set = 0, binding = 1) readonly buffer billboard_buffer
{
    BillboardBufferData data[];
} BillboardBuffer;

const vec2 VERTEX_COORDS[6] = vec2[](
    vec2(-1.0, -1.0),
    vec2(-1.0,  1.0),
    vec2( 1.0, -1.0),
    vec2( 1.0, -1.0),
    vec2(-1.0,  1.0),
    vec2( 1.0,  1.0)
);

layout (location = 0) out vec2 frag_offset;

const float BILLBOARD_RADIUS = 0.25;

void main() {
    frag_offset = VERTEX_COORDS[gl_VertexIndex];

    vec3 camera_right_world = {
        CameraBuffer.view_matrix[0][0],
        CameraBuffer.view_matrix[1][0],
        CameraBuffer.view_matrix[2][0],
    };
    vec3 camera_up_world = {
        CameraBuffer.view_matrix[0][1],
        CameraBuffer.view_matrix[1][1],
        CameraBuffer.view_matrix[2][1],
    };

    BillboardBufferData data = BillboardBuffer.data[gl_InstanceIndex];
    vec3 billboard_position = data.position.xyz;
    vec3 position_world = billboard_position
        + BILLBOARD_RADIUS * frag_offset.x * camera_right_world
        + BILLBOARD_RADIUS * frag_offset.y * camera_up_world;

    gl_Position = CameraBuffer.projection_matrix * CameraBuffer.view_matrix * vec4(position_world, 1.0);
}
