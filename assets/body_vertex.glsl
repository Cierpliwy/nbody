#version 150

in vec3 position;
in vec3 color;
in float radius;

out VertexData
{
  vec4 position;
  vec3 color;
  float radius;
} outData;

uniform mat4 mvp;

void main() {
    vec4 p = mvp * vec4(position, 1.0);
    vec4 p2 = mvp * vec4(position + vec3(radius, 0.0, 0.0), 1.0);
    outData.position = p / p.w;
    outData.color = color;
    outData.radius = length(p2 / p2.w - p / p.w);
}