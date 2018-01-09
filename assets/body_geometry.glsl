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
out float fog;

uniform vec2 res;
uniform mat4 projection;
uniform vec4 look_at;
uniform float focus;
uniform float far;
uniform float near;

void main()
{
    center = inData[0].position;
    color = inData[0].color;

    float focus_distance = (1.0 - clamp(abs(look_at.z-center.z) / focus, 0.0, 1.0));
    radius = inData[0].radius;

    vec4 s1 = projection * center;
    s1 /= s1.w;
    vec4 s2 = projection * (center + vec4(1.0, 0.0, 0.0, 0.0));
    s2 /= s2.w;
    blur = (s2.x - s1.x) * res.x / res.y * focus_distance;

    fog = (abs((-center.z - near) / (far - near) - 0.5) * 2.0);
    fog = 1.0 - fog*fog*fog;

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