#version 450
#extension GL_ARB_separate_shader_objects : enable

// The texture descriptor.
layout(set = 0, binding = 0) uniform sampler2D u_Albedo;

layout(early_fragment_tests) in;

// Rasterized position of fragment.
layout(location = 0) in vec3 i_FragPos;
// The surface normal.
layout(location = 1) in vec3 i_Normal;
// The uv (used for mapping texture.
layout(location = 2) in vec2 i_Uv;

// The output color (rgba).
layout(location = 0) out vec4 o_Target;

void main() {
    // Get frag color from texture.
    vec4 tex_color = texture(u_Albedo, i_Uv);

    // TODO: Modify fragment to add pbr lighting etc.
    //

    // Set final output color;
    o_Target = tex_color;
}
