#version 330 core

in vec4 vertexColor;
out vec4 FragColor;

uniform float colorMultiplier;

void main() {
   FragColor = colorMultiplier * vertexColor;
}
