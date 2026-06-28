

float rand(vec3 x) { 
    return fract(sin(dot(x, vec3(11.1191, 78.233, 36.15861))) * 43758.5453);
}

// hash 2d point to 3d random values
vec3 hash3(vec2 x) {
    return vec3(
        rand(vec3(x.xy, 0.257)),
        rand(vec3(x.xy, 15.65)),
        rand(vec3(x.xy, 199.1)));
}

/// Pick nearest representative point
vec2 noiseS2(in vec2 x, float u)
{
    vec2 p = floor(x);
    vec2 f = fract(x);

    float m = 100.0;
    vec2 ot = vec2(0.0, 0.0);
    for (int j= -2; j <= 2; j++) {
        for (int i = -2; i <= 2; i++) {
            vec2 g = vec2(float(i), float(j));
            vec3 o = hash3(p + g) * vec3(u, u, 1.0);
            vec2 r = g - f + o.xy;
            float d = dot(r, r);
            if (d < m) {
                ot = p + g;
            }
            m = min(m, d);
        }
    }
    return ot;
}

float turbulence(vec3 x, int iters, float fratio, float wratio) {
   float y = 0.0;
   float w = 1.0;
   float s = 1.0;
   float m = 0.0;
   for (int i = 0; i < 32; i++) {
      if (i >= iters) break;
      y += w * abs(noise3(x * s));
      m += w;
      s *= fratio;
      w *= wratio;
   }
   return y / m;
}

// // Calculate distance to an edge
// float edgeDist = abs(uv.x - 0.5);

// // Find how much the distance changes per pixel
// float width = fwidth(edgeDist);

// // Create a sharp 1-pixel border using smoothstep
// float outline = smoothstep(0.0, width, edgeDist);

void main() {

    vec2 p = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / u_resolution.y;
    float scale = 10.0;
    float twist = 7.0;
    float rotate = log(length(p)) * twist;
    float v = noise3(vec3(p * rotate2D(rotate) * scale * rotate2D(-0.5 * u_time), u_time * 0.05));
    gl_FragColor = vec4(v, v, v, 1.0);

}
