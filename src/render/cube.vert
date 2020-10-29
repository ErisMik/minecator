in vec3 co3;
in vec3 nor;

// out vec3 v_nor;
// out vec3 v_cam_dir;
out vec3 v_color;

uniform mat4 projection;
uniform mat4 view;
// uniform mat4 model;
uniform float aspect_ratio;

vec3[24] VERTEX_COLORS = vec3[](
    vec3(1., 0., 0.), vec3(1., 0., 0.), vec3(1., 0., 0.), vec3(1., 0., 0.),
    vec3(0., 1., 0.), vec3(0., 1., 0.), vec3(0., 1., 0.), vec3(0., 1., 0.),
    vec3(0., 0., 1.), vec3(0., 0., 1.), vec3(0., 0., 1.), vec3(0., 0., 1.),
    vec3(1., 0., 1.), vec3(1., 0., 1.), vec3(1., 0., 1.), vec3(1., 0., 1.),
    vec3(0., 1., 1.), vec3(0., 1., 1.), vec3(0., 1., 1.), vec3(0., 1., 1.),
    vec3(1., 1., 0.), vec3(1., 1., 0.), vec3(1., 1., 0.), vec3(1., 1., 0.)
);

void main() {
  vec4 p = view * vec4(co3, 1.);
  gl_Position = projection * p;

  // v_nor = mat3(view) * nor;
  // v_cam_dir = p.xyz;
  v_color = VERTEX_COLORS[gl_VertexID];
}
