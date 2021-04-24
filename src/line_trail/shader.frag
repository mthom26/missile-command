#version 450

layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform LineMaterial_color {
    vec4 u_color;
};

void main() {
    o_Target = vec4(u_color.r, u_color.g, u_color.b, uv.y);
}
