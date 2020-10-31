#version 300 es
precision highp float;
precision highp int;
in vec4 coord;
out vec4 color;
vec4 invGamma(vec4 color)
{
    return vec4(pow(color.rgb, vec3(1.0 / 2.2)), color.a);
}
vec4 gamma(vec4 color)
{
    return vec4(pow(color.rgb, vec3(2.2)), color.a);
}
// Taken from https://en.wikipedia.org/wiki/Mandelbrot_set
float iter(vec2 pos)
{
    vec2 x = pos;
    int iters = 0;
    int max_iters = 100;
    while (x.x * x.x + x.y * x.y < 2.0 && iters < max_iters) {
        float xt = x.x * x.x - x.y * x.y + pos.x;
        x.y = 2.0 * x.x * x.y + pos.y;
        x.x = xt;
        iters++;
    }
    return float(iters) / float(max_iters);
}
void main()
{
    color = vec4(vec3(iter(coord.xy)), 1.0);
}
