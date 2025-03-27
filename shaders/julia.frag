#ifdef GL_ES
precision highp float;
precision highp int;
#endif

varying vec2 v_texcoord;

uniform vec2 u_screen_dimensions;
uniform vec2 u_viewport_min;
uniform vec2 u_viewport_max;
uniform int u_max_iterations;
uniform float u_escape_radius;
uniform float u_julia_a;


vec3 get_color(float t) {
    vec3 dark_blue = vec3(0.0, 7.0/255.0, 100.0/255.0);
    vec3 blue = vec3(32.0/255.0, 107.0/255.0, 203.0/255.0);
    vec3 white = vec3(237.0/255.0, 255.0/255.0, 255.0/255.0);
    vec3 orange = vec3(255.0/255.0, 170.0/255.0, 0.0/255.0);
    vec3 almost_black = vec3(0.0, 2.0/255.0, 0.0);

    if (t < 0.16) {
        float blend = t / 0.16;
        return mix(dark_blue, blue, blend);
    } else if (t < 0.42) {
        float blend = (t - 0.16) / 0.26;
        return mix(blue, white, blend);
    } else if (t < 0.6425) {
        float blend = (t - 0.42) / 0.2225;
        return mix(white, orange, blend);
    } else if (t < 0.8575) {
        float blend = (t - 0.6425) / 0.215;
        return mix(orange, almost_black, blend);
    } else {
        return almost_black;
    }
}

void main() {
    vec2 z = vec2(
        u_viewport_min.x + v_texcoord.x * (u_viewport_max.x - u_viewport_min.x),
        u_viewport_min.y + v_texcoord.y * (u_viewport_max.y - u_viewport_min.y)
    );

    vec2 c = 0.7885 * vec2(cos(u_julia_a), sin(u_julia_a));

    int iterations = 0;
    for (int i = 0; i < 10000; i++) {
        if (i >= u_max_iterations) break;
        if (dot(z, z) > u_escape_radius) break;

        vec2 z_new = vec2(
            z.x * z.x - z.y * z.y + c.x,
            2.0 * z.x * z.y + c.y
        );
        z = z_new;
        iterations++;
    }

    float smoothed_t = float(iterations) + 1.0 - log(log(length(z))) / log(2.0);
    float t = smoothed_t / float(u_max_iterations);
    vec3 color = get_color(t);

    gl_FragColor = vec4(color, 1.0);
}
