#version 450

layout(push_constant) uniform PushConstants {
    vec2 screen;
} pcs;

layout(location = 0) in vec2 inPos;
layout(location = 1) in vec2 inUV;
layout(location = 2) in vec4 inColor;

layout(location = 0) out vec2 fragUV;
layout(location = 1) out vec4 fragColor;

void main() {
    float x = (2.0 * (inPos.x / pcs.screen.x)) - 1.0;
    float y = (2.0 * (inPos.y / pcs.screen.y)) - 1.0;
    gl_Position = vec4(x, y, 0.0, 1.0);
    fragUV = inUV;
    fragColor = inColor;
}
