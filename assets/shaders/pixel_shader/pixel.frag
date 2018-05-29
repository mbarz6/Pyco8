#version 440 core
out vec4 FragColor;  

uniform int color;

void main()
{
    vec3 color_vec = (0.0f, 0.0f, 0.0f); // default to black
    if (color == 1) { // white
        color_vec = (1.0f, 1.0f, 1.0f);
    } 

    FragColor = vec4(color_vec, 1.0f);
}