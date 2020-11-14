#version 450 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;


layout (location = 0) uniform vec4 screenSizes;

layout (std430, binding=2) buffer shader_data
{
	vec4 values[];
} vData;

layout (location = 0) out vec3 vColor;

void main()
{
	int vertId = gl_VertexID / 6;
	int v = gl_VertexID % 6;
	vec2 pp = vec2(0.5f, 0.5f);
	if(v == 0 || v == 1 || v == 5)
	{
		pp.y = -0.5f;
	}
	if(v == 0 || v == 4 || v == 5)
	{
		pp.x = -0.5f;
	}
	vec3 c = vec3(pp.xy + 0.5f, 0.0f);

	pp *= screenSizes.y;
	pp.xy += vData.values[vertId].xy;
	pp.xy /= screenSizes.zw;
	gl_Position = vec4(vec3(pp.xy, 1.0f) , 1.0);
	vColor = c;
}