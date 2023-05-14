#version 460 core

in vec2 vTex;
out vec4 fCol;

uniform sampler2D mat_texture;

void main() {
    fCol = texture(mat_texture, vTex);
}
