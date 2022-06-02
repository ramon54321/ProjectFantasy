    #version 450

    layout(location = 0) in vec2 position;
    layout(location = 0) out vec2 f_position;

    layout(set = 0, binding = 0) uniform UniformBufferObject {
	mat4 projection;
    } ubo;

    void main() {
	f_position = vec2(position.x, -position.y);
        gl_Position = ubo.projection * vec4(position, -1.0, 1.0);
    }
