#version 150

in vec4 position;
in vec4 center;
in vec3 color;
in float radius;
in float blur;

uniform mat4 projection;

void main() {
    vec4 diff = position-center;
    float len = length(diff);
    if (len < radius) {
        float height = sqrt(radius*radius - diff.x*diff.x - diff.y*diff.y);
        vec4 screen = projection * vec4(position.x, position.y, position.z + height, position.w);
        screen /= screen.w;
        gl_FragColor = vec4(color * height / radius, (radius - len) * (1 + 128.0 * blur));
        gl_FragDepth = screen.z * 0.5 + 0.5;
    } else {
        gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);
        gl_FragDepth = 1.0;
    }
}