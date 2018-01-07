#version 150

in vec4 position;
in vec4 center;
in vec3 color;
in float radius;

uniform vec2 res;

void main() {
    vec4 diff = center - position;
    diff.y *= res.y/res.x;
    float len = length(diff);
    if (len < radius) {
        float height = sqrt(radius*radius - len*len);
        float relativeHeight = height / len;
        gl_FragDepth = (position.z - height + 1.0) / 2.0;
        gl_FragColor = vec4(color * relativeHeight, (radius - len) * res.x / 8.0);
    } else {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
        gl_FragDepth = 1.0;
    }
}