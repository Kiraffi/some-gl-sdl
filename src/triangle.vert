#version 450 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

layout (location = 0) uniform vec4 posRotSize;

layout (location = 0) out vec3 vColor;

void main()
{
	gl_Position = vec4(position + posRotSize.xyz, 1.0);
	vColor = color;
}