#version 150

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

in VertexData
{
  vec4 position;
  vec3 color;
  float radius;
} inData[];

out vec4 position;
out vec4 center;
out vec3 color;
out float radius;
out float blur;

uniform vec2 res;
uniform mat4 projection;

void main()
{
    center = inData[0].position;
    color = inData[0].color;
    radius = inData[0].radius;

    vec4 s1 = projection * center;
    s1 /= s1.w;
    vec4 s2 = projection * (center + vec4(1.0, 0.0, 0.0, 0.0));
    s2 /= s2.w;
    blur = (s2.x - s1.x) * res.x / res.y * 2.0;

    position = center + vec4(-radius, -radius, 0.0, 0.0);
    gl_Position = projection * position;
    EmitVertex();

    position = center + vec4(radius, -radius, 0.0, 0.0);
    gl_Position = projection * position;
    EmitVertex();

    position = center + vec4(-radius, radius, 0.0, 0.0);
    gl_Position = projection * position;
    EmitVertex();

    position = center + vec4(radius, radius, 0.0, 0.0);
    gl_Position = projection * position;
    EmitVertex();

    EndPrimitive();
}