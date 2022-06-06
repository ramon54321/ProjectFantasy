    #version 450

    layout(location = 0) in vec2 position;
    layout(location = 0) out vec3 f_color;

    layout(set = 0, binding = 0) uniform UniformBufferObject {
	mat4 view;
	mat4 projection;
    } ubo;

    void main() {
	float red = int(gl_VertexIndex) % 3;
	f_color = vec3(red / 3.0, 1.0 - red / 3.0, 1.0);
        gl_Position = ubo.projection * ubo.view * vec4(position, -1.0, 1.0);
    }
