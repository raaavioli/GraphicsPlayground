#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;
layout (location = 2) in vec2 uv;

out vec2 frag_uv;

uniform mat4 Perspective;
uniform mat4 View;

void main()
{
    mat4 MVP = Perspective * View;
    frag_uv = uv;
    gl_Position = MVP * vec4(position, 1.0f);
}