#version 330 core

in vec2 v_uv;
out vec4 frag_color;

uniform sampler2D u_texture;
uniform vec4 u_color;

void main() {
	vec4 tex_color = texture(u_texture, v_uv);
	if (tex_color.a <= 0.1) {
		discard;
	}

	frag_color = tex_color * u_color;
}
