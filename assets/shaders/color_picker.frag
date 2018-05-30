#version 440 core
out vec4 FragColor;  

uniform uint color;

void main()
{
    if (color == 1) { // white
        FragColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    } else { // default to black
        FragColor = vec4(0.0f, 0.0f, 0.0f, 1.0f);
    }
}