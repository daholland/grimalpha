#version 150

in vec3 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform mat4 matrix;

void main() {

v_tex_coords = tex_coords;

gl_Position = matrix * vec4(position, 1.0);

}