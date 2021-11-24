#version 450 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

struct VertexData
{
    vec2 pos;
    vec2 size;
    vec2 uv;
    uint col;
    float tmp;
};

layout (std140, binding=0) uniform frame_data
{
    vec2 screenSizes;
    vec2 padding;
};

layout (std430, binding=1) buffer shader_data
{
    VertexData values[];
} vData;


layout (location = 0) out vec4 vColor;
layout (location = 1) out vec2 vTexCoord;

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

    vTexCoord = pp + 0.5f;
    vTexCoord.y = 1.0f - vTexCoord.y;
    vTexCoord.x += vData.values[(vertId)].uv.x;
    vTexCoord.x /= (128.0f-32.0f);

    pp.xy *= vData.values[vertId].size;
    pp.xy += vData.values[vertId].pos;
    //pp.xy += 0.5f;

    pp.xy = (pp.xy + 0.5f) / (screenSizes.xy * 0.5f) - 1.0f;
    gl_Position = vec4(vec3(pp.xy, 1.0f) , 1.0);
    uint col = vData.values[vertId].col;
    uvec4 cu = uvec4((col & 255u), (col >> 8) & 255, (col >> 16) & 255, (col >> 24) & 255);
    vColor = vec4(cu) / 255.0f;
}