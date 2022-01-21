#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform vec3 color;

out VS_OUTPUT {
    vec2 TexCoord;
    vec3 Color;
} OUT;

void main()
{
    gl_Position = projection * view * model * vec4(Position, 1.0);
    OUT.TexCoord = TexCoord;
    OUT.Color = color;
}