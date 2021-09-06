#version 450 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

layout (location = 0) uniform vec4 screenSizes;

/*
layout (std430, binding=2) buffer shader_data
{
	uvec4 values[];
} vData;
*/

struct DataValue
{
	vec2 pos;
	uint col;
	float sz;
};

layout (std140, binding=2) uniform shader_data
{
	uvec4 values[1024];
} vData;


layout (location = 0) out vec4 vColor;

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

	pp *= uintBitsToFloat(vData.values[vertId].w);
	pp.xy += uintBitsToFloat(vData.values[vertId].xy);
	//pp.xy += 0.5f;
	
	pp.xy = (pp.xy + 0.5f) / (screenSizes.zw * 0.5f) - 1.0f;
	gl_Position = vec4(vec3(pp.xy, 1.0f) , 1.0);
	uint col = vData.values[vertId].z;
	uvec4 cu = uvec4((col & 255u), (col >> 8) & 255, (col >> 16) & 255, (col >> 24) & 255);
	vColor = vec4(cu) / 255.0f;
}