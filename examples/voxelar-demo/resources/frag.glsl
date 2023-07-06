#version 330 core

in vec4 vertexColor;
out vec4 FragColor;

uniform vec3 colors[3];

void main() {
    vec3 reconstructed = vec3(colors[0].x, colors[1].y, colors[2].z);
    FragColor = vec4(reconstructed, 1.0);
}
