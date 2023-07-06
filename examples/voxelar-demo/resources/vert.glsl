#version 330 core

layout (location = 0) in vec3 vpos;
layout (location = 1) in vec3 vcol;

out vec4 vertexColor;

void main() {
    gl_Position = vec4(vpos, 1.0);
    vertexColor = vec4(vcol, 1.0);
}
