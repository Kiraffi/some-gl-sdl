#version 450 core

layout (location = 0) out vec2 vTexCoord;
void main()
{
    int v = gl_VertexID;
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

    pp *= 2.0f;

    gl_Position = vec4(vec3(pp.xy, 1.0f) , 1.0);
}