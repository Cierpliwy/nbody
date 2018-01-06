#version 150

in vec4 position;
in vec4 center;
in vec3 color;
in float radius;

void main() {
    float len = length(center-position);
    if (len < radius) {
        float height = sqrt(radius*radius - len*len);
        float relativeHeight = height / len;
        gl_FragDepth = (position.z - height + 1.0) / 2.0;
        gl_FragColor = vec4(color*(relativeHeight*0.5+0.5), relativeHeight*relativeHeight);
    } else {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
        gl_FragDepth = 1.0;
    }
}