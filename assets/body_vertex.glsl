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

uniform mat4 projection;
uniform mat4 view;

void main() {
    outData.position = view * vec4(position, 1.0);
    outData.color = color;
    outData.radius = radius;
}