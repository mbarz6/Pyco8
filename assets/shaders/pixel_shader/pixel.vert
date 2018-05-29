#version 440 core
layout (location = 0) in vec2 a_pos;

uniform float offset_x;
uniform float offset_y;

out vec3 color;

void main()
{
    gl_Position = vec4(a_pos.x + offset_x, a_pos.y + offset_y, 0.0, 1.0);
}  