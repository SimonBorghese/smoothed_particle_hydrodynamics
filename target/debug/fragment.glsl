#version 460 core

in vec2 oTex;

out vec4 FragColor;

layout (location = 2) uniform sampler2D tex;

void main(){
    FragColor = texture(tex, oTex);
}