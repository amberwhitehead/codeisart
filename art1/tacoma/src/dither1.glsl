
// white noise (noise from [0, 1] uncorrelated with neighbor pixels)
float rand(vec3 x) { 
    return fract(sin(dot(x, vec3(11.1191, 78.233, 36.15861))) * 43758.5453);
}

float fbm(vec3 x, int iters, float fratio, float wratio) {
   float y = 0.0;
   float w = 1.0;
   float s = 1.0;
   float m = 0.0;
   for (int i = 0; i < 32; i++) {
      if (i >= iters) break;
      y += w * noise3(x * s);
      m += w;
      s *= fratio;
      w *= wratio;
   }
   return y / m;
}

void main()
{
    vec3 p = gl_FragCoord.xyz / u_resolution.xyy;
    vec3 p0 = p;
    p.z += u_time * 0.05;
    float f = fbm(p * 4.0, 8, 2.0, 0.5);
    float w = rand(p.x < 0.5 ? p : p0);
    float n = w > (f * 1.2 + 0.6) ? 1.0 : 0.0;
    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0) * n;
}
