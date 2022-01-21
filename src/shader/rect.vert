#version 330 core

uniform vec2 cam;
uniform vec2 pos;
uniform vec2 dim;
uniform vec4 color;

vec2 calculate_coords() {
    vec2 res_pos = pos / cam;
    vec2 res_dim = dim / cam;

    vec2 res = vec2(2.0 * res_pos.x - 1.0, 1.0 - 2.0 * res_pos.y);

    float width = 2.0 * res_dim.x;
    float height = 2.0 * res_dim.y;

    if(gl_VertexID == 1) {
        res.x = res.x + width;
    } else if(gl_VertexID == 2) {
        res.x = res.x + width;
        res.y = res.y - height;
    } else if(gl_VertexID == 3) {
        res.y = res.y - height;
    }

    return res;
}

out VS_OUTPUT {
    vec4 Color;
} OUT;

void main()
{
    vec2 pos = calculate_coords();
    gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
    OUT.Color = color;
}