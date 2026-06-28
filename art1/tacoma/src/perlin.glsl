
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
   float n = noise3(p * 10.0);
   color = vec4(0.5, 0.5, 0.5, 1.0) + vec4(0.5, 0.5, 0.5, 0.0) * n;
   gl_FragColor = color;
}
