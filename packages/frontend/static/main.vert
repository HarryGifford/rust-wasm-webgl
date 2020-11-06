#version 300 es
precision highp float;
precision highp int;
in vec4 position;
in vec4 normal;
out vec4 coord;

uniform mat4 view_proj;

void main()
{
    gl_Position = view_proj * position;
    coord = position;
}
