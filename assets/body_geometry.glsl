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

void main()
{
    center = inData[0].position;
    color = inData[0].color;
    radius = inData[0].radius;

    position = center + vec4(-radius, -radius, 0.0, 0.0);
    gl_Position = position;
    EmitVertex();

    position = center + vec4(radius, -radius, 0.0, 0.0);
    gl_Position = position;
    EmitVertex();

    position = center + vec4(-radius, radius, 0.0, 0.0);
    gl_Position = position;
    EmitVertex();

    position = center + vec4(radius, radius, 0.0, 0.0);
    gl_Position = position;
    EmitVertex();

    EndPrimitive();
}