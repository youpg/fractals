attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color;

varying vec2 v_texcoord;

uniform vec2 u_screen_dimensions;

void main() {
    v_texcoord = vec2(position.x / u_screen_dimensions.x, position.y / u_screen_dimensions.y);
    gl_Position = vec4(
        (position.x / u_screen_dimensions.x) * 2.0 - 1.0,
        (1.0 - (position.y / u_screen_dimensions.y)) * 2.0 - 1.0,
        0.0,
        1.0
    );
}
