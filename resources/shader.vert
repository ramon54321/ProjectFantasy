#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 texture_coordinates;
layout(location = 0) out vec3 f_color;
layout(location = 1) out vec2 f_textureCoordinates;

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 projection;
} ubo;

struct StorageData {
    uint frame;
};

layout(set = 2, binding = 0) buffer StorageBuffer {
    StorageData objects[];
} storage;

void main() {
    float red = int(gl_VertexIndex) % 3;
    //f_color = vec3(red / 3.0, 1.0 - red / 3.0, 1.0);
    uint quadIndex = gl_VertexIndex / 6;
    f_color = vec3(red / 3.0, 1.0 - red / 3.0, float(storage.objects[quadIndex].frame) / 10.0);
    f_textureCoordinates = texture_coordinates;
    gl_Position = ubo.projection * ubo.view * vec4(position, -1.0, 1.0);
}
