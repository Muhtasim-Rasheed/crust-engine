#version 330 core

in vec2 v_uv;
out vec4 frag_color;

uniform sampler2D u_texture;
uniform vec4 u_color;

uniform int u_effects[32];
uniform float u_effect_values[32];
uniform int u_effects_count;

struct Effect {
	int type;
	float value;
};

Effect get_effect(int index) {
	return Effect(u_effects[index], u_effect_values[index]);
}

void main() {
	vec4 tex_color = texture(u_texture, v_uv);
	if (tex_color.a <= 0.1) {
		discard;
	}

	for (int i = 0; i < u_effects_count; ++i) {
		Effect effect = get_effect(i);
		if (effect.type == 0) // Brightness effect
			tex_color.rgb += vec3(clamp(effect.value / 100.0, -1.0, 1.0));
		// Add others later
	}

	frag_color = tex_color * u_color;
}
