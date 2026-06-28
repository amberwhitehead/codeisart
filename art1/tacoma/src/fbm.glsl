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
   vec4 color = vec4(0.0, 0.0, 0.0, 1.0);
   vec2 st = gl_FragCoord.xy / u_resolution.xy;
   vec3 p = gl_FragCoord.xyz / u_resolution.x;
   if (p.x > 0.5) {
      p.z = u_time * 0.02;
   } else {
      p.xy += u_time * 0.02;
   }
   float n = fbm(p * 8.0, 8, 2.0, 0.5) * 0.5 + 0.5;
   color = vec4(1.0, 1.0, 1.0, 1.0) * n;

   gl_FragColor = color;
}
