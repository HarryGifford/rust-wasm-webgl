#version 300 es
precision highp float;
precision highp int;
in vec4 position;
out vec4 coord;

void main()
{
    gl_Position = position;
    coord = position;
}
