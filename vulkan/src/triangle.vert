#version 450
layout (location = 0) out vec4 vColor;

vec4 positions[3] = vec4[]
(
	vec4(0.0, -0.5,0.0,1.0),
	vec4(0.5, 0.5,0.0,1.0),
	vec4(-0.5, 0.5,0.0,1.0)
);

void main() 
{
	vec4 col = vec4(0.0f, 0.0f, 0.0f, 1.0f);
	col.x = gl_VertexIndex == 0 ? 1.0f : 0.0f;
	col.y = gl_VertexIndex == 1 ? 1.0f : 0.0f;
	col.z = gl_VertexIndex == 2 ? 1.0f : 0.0f;
	
	gl_Position = positions[gl_VertexIndex];
	vColor = col;
}