// in vec3 v_nor;
// in vec3 v_cam_dir;
in vec3 v_color;
// uniform mat4 view;

out vec4 frag;

// uniform samplerCube environment;

void main() {
  // vec3 bounce = reflect(v_cam_dir, normalize(v_nor)); // view space
  // bounce = inverse(mat3(view)) * bounce;
  // vec3 color = texture(environment, bounce).rgb;

  frag = vec4(v_color, 1.);
}
