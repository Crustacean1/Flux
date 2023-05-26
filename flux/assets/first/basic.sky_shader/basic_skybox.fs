#version 420 core

in vec2 vTex;
out vec4 fCol;

uniform sampler2D mat_texture;

void main(){
    fCol = texture(mat_texture, vTex);
    //fCol = vec4(vTex, 0.0,1.0);
}


