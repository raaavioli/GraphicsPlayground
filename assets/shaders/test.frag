#version 330 core

in vec2 frag_uv;

out vec4 out_color;

uniform sampler2D Texture;

void main()
{
    out_color = vec4(texture(Texture, vec2(frag_uv.x, 1 - frag_uv.y)).rgb, 1.0);
}