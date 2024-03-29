#version 450 core
layout (location = 0) in vec2 vTexCoord;

layout (location = 0) out vec4 outColor;

layout (binding = 0) uniform sampler2D ourTexture;

void main()
{
    outColor = texture(ourTexture, vTexCoord);
}