#version 140

in vec2 fragPosition;
out vec4 color;

void main() {
    color = vec4(fragPosition.xy * 2.0, 0.0, 1.0);
}
