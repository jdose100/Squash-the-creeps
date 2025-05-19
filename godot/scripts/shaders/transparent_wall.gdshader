shader_type spatial;
render_mode blend_mix;

uniform float alpha;

void fragment() {
	// Считаем альфу.
	float new_alpha;
	if (alpha <= 4.1)
		new_alpha = normalize(alpha) - 0.17;
	else
		new_alpha = 1.0;
	ALPHA = new_alpha;

	// Устанавливаем albedo.
	ALBEDO = vec3(0.92, 0.86, 0.0);
}

