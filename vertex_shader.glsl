#version 140
in vec2 position;
uniform float t;
out vec2 pos;
void main() {
	pos = position;
	pos.x += t;
	gl_Position = vec4(pos, 0.0, 1.0);
}

