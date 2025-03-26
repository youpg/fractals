#ifdef GL_ES
precision highp float;
precision highp int;
#endif

varying vec2 v_texcoord;

uniform vec2 u_screen_size;
uniform vec2 u_view_min;
uniform vec2 u_view_max;
uniform int u_max_iterations;

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
    vec2 c = vec2(
        u_view_min.x + v_texcoord.x * (u_view_max.x - u_view_min.x),
        u_view_min.y + v_texcoord.y * (u_view_max.y - u_view_min.y)
    );

    vec2 z = vec2(0.0);
    int iterations = 0;

    for (int i = 0; i < u_max_iterations; i++) {
        if (z.x * z.x + z.y * z.y > 4.0) {
            break;
        }
        float x_new = z.x * z.x - z.y * z.y + c.x;
        z.y = 2.0 * z.x * z.y + c.y;
        z.x = x_new;
        iterations++;
    }

    if (iterations >= u_max_iterations) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        float t = float(iterations) / float(u_max_iterations);
        vec3 color = get_color(t);
        gl_FragColor = vec4(color, 1.0);
    }
}
