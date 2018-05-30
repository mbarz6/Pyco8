#version 440 core
layout (location = 0) in vec2 a_pos;

uniform float offsetx;
uniform float offsety;

void main()
{
    gl_Position = vec4(a_pos.x + offsetx, a_pos.y + offsety, 0.0, 1.0);
}