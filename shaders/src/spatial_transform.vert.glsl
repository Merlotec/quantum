#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(push_constant) uniform u_TransformBlock {
    mat4 u_MvpTransform;
};

layout(location = 0) in vec3 i_Vertex;
layout(location = 1) in vec3 i_Normal;
layout(location = 2) in vec2 i_Uv;

layout(location = 0) out vec3 o_FragPos;
layout(location = 1) out vec3 o_Normal;
layout(location = 2) out vec2 o_Uv;

void main() {
    // The screenspace position of the vertex.
    vec4 screen_pos = u_MvpTransform * vec4(i_Vertex, 0);

    // Sends a liner interpolation of all the vertex positions to the fragment shader.
    o_FragPos = i_Vertex;
    // Automatic interpolation
    o_Normal = i_Normal;
    // Automatic interpolation
    o_Uv = i_Uv;

    // Give the resultant screenspace vetex position to the pipeline.
    gl_Position = screen_pos;
}
