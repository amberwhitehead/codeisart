

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

void main() {

    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / u_resolution.y;
    float zoom = cos(u_time * 0.1) * 100.0 + 1.0;
    // pick nearest representative point
    vec2 op = noiseS2(uv * zoom, 1.0);
    // use that single point to lookup in noise
    vec3 opScaled = vec3(op.xy * 0.01635 + u_time * 0.1, u_time * 0.5);
    float p = turbulence(opScaled, 8, 2.0, 0.5);
    gl_FragColor = vec4(p, p, p, 1.0);

}
