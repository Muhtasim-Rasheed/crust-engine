#version 330 core

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_uv;

out vec2 v_uv;

uniform mat4 u_projection;
uniform mat4 u_model;

void main() {
	gl_Position = u_projection * u_model * vec4(a_position, 0.0, 1.0);

	v_uv = a_uv;
}
