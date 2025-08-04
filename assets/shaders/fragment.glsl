#version 330 core

in vec2 v_uv;
out vec4 frag_color;

uniform sampler2D u_texture;
uniform vec4 u_color;

uniform int u_effects[32];
uniform float u_effect_values[32];
uniform int u_effects_count;

const int BRIGHTNESS     =  0;
const int GHOST          =  1;
const int HUE            =  2;
const int SATURATION     =  3;
const int SEPIA          =  4;
const int GRAYSCALE_AVG  =  5;
const int GRAYSCALE_WGT  =  6;
const int INVERT         =  7;
const int MULT           =  8;
const int MULT_R         =  9;
const int MULT_G         = 10;
const int MULT_B         = 11;
const int MULT_A         = 12;
const int ADD            = 13;
const int ADD_R          = 14;
const int ADD_G          = 15;
const int ADD_B          = 16;
const int ADD_A          = 17;

struct Effect {
	int type;
	float value;
};

Effect get_effect(int index) {
	return Effect(u_effects[index], u_effect_values[index]);
}

vec3 rgb2hsv(vec3 c) {
	vec4 K = vec4(0.0, -1.0/3.0, 2.0/3.0, -1.0);
	vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
	vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

	float d = q.x - min(q.w, q.y);
	float e = 1.0e-10;
	return vec3(
		abs(q.z + (q.w - q.y) / (6.0 * d + e)), // H
		d / (q.x + e),                          // S
		q.x                                     // V
	);
}

vec3 hsv2rgb(vec3 c) {
	vec3 p = abs(fract(c.xxx + vec3(0.0, 1.0/3.0, 2.0/3.0)) * 6.0 - 3.0);
	return c.z * mix(vec3(1.0), clamp(p - 1.0, 0.0, 1.0), c.y);
}

vec3 apply_hue(vec3 color, float hue) {
	vec3 hsv = rgb2hsv(color);
	hsv.x = mod(hsv.x + hue / 360.0, 1.0);
	return hsv2rgb(hsv);
}

vec3 apply_saturation(vec3 color, float saturation) {
	vec3 hsv = rgb2hsv(color);
	hsv.y *= clamp(saturation / 100.0 + 1.0, 0.0, 2.0);
	return hsv2rgb(hsv);
}

vec3 apply_sepia(vec3 color, float amount) {
	amount = clamp(amount, 0.0, 1.0);

	vec3 sepiaColor = vec3(
		dot(color, vec3(0.393, 0.769, 0.189)), // red
		dot(color, vec3(0.349, 0.686, 0.168)), // green
		dot(color, vec3(0.272, 0.534, 0.131))  // blue
	);

	return mix(color, sepiaColor, amount);
}

vec3 apply_grayscale_averaged(vec3 color, float amount) {
	vec3 averaged = vec3(color.r / 3.0 + color.g / 3.0 + color.b / 3.0);
	return mix(color, averaged, amount);
}

vec3 apply_grayscale_weighted(vec3 color, float amount) {
	vec3 weighted = vec3(color.r * 0.299 + color.g * 0.587 + color.b * 0.114);
	return mix(color, weighted, amount);
}

void main() {
	vec4 tex_color = texture(u_texture, v_uv);

	for (int i = 0; i < u_effects_count; ++i) {
		Effect effect = get_effect(i);
		if (effect.type == BRIGHTNESS)
			tex_color.rgb += vec3(clamp(effect.value / 100.0, -1.0, 1.0));
		else if (effect.type == GHOST)
			tex_color.a *= clamp(1.0 - effect.value / 100.0, 0.0, 1.0);
		else if (effect.type == HUE)
			tex_color.rgb = apply_hue(tex_color.rgb, effect.value);
		else if (effect.type == SATURATION)
			tex_color.rgb = apply_saturation(tex_color.rgb, effect.value);
		else if (effect.type == SEPIA)
			tex_color.rgb = apply_sepia(tex_color.rgb, effect.value);
		else if (effect.type == GRAYSCALE_AVG)
			tex_color.rgb = apply_grayscale_averaged(tex_color.rgb, clamp(effect.value / 100.0, 0.0, 1.0));
		else if (effect.type == GRAYSCALE_WGT)
			tex_color.rgb = apply_grayscale_weighted(tex_color.rgb, clamp(effect.value / 100.0, 0.0, 1.0));
		else if (effect.type == INVERT)
			tex_color.rgb = mix(tex_color.rgb, 1.0 - tex_color.rgb, clamp(effect.value / 100.0, 0.0, 1.0));
		else if (effect.type == MULT)
			tex_color.rgb *= effect.value;
		else if (effect.type == MULT_R)
			tex_color.r *= effect.value;
		else if (effect.type == MULT_G)
			tex_color.g *= effect.value;
		else if (effect.type == MULT_B)
			tex_color.b *= effect.value;
		else if (effect.type == MULT_A)
			tex_color.a *= effect.value;
		else if (effect.type == ADD)
			tex_color.rgb += effect.value;
		else if (effect.type == ADD_R)
			tex_color.r += effect.value;
		else if (effect.type == ADD_G)
			tex_color.g += effect.value;
		else if (effect.type == ADD_B)
			tex_color.b += effect.value;
		else if (effect.type == ADD_A)
			tex_color.a += effect.value;
	}

	if (tex_color.a <= 0.1) {
		discard;
	}

	frag_color = tex_color * u_color;
}
