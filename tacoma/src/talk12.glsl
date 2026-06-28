
float rand(vec3 x) { 
    return fract(sin(dot(x, vec3(11.1191, 78.233, 36.15861))) * 43758.5453);
}

float sdCircle( vec2 p, float r )
{
   return length(p) - r;
}


float sdRoundedBox( in vec2 p, in vec2 b, in vec4 r )
{
   r.xy = (p.x>0.0)?r.xy : r.zw;
   r.x  = (p.y>0.0)?r.x  : r.y;
   vec2 q = abs(p)-b+r.x;
   return min(max(q.x,q.y),0.0) + length(max(q,0.0)) - r.x;
}


vec2 rotate(vec2 v, float angle) {
   float s = sin(angle);
   float c = cos(angle);
   mat2 m = mat2(c, -s, s, c);
   return m * v;
}


void main() {
 vec2 p0 = ((gl_FragCoord.xy / u_resolution.xy) - 0.5) * 2.0;
 p0.x *= u_resolution.x / u_resolution.y;
 vec2 p = p0;
 float c = 0.0;
 for (int i = 0; i < 20; i++) {
   float s = 10.0;
   vec2 o = vec2(
     noise3(vec3(float(i), 17.3, 1.0)) * 2.0 - 1.0,
     noise3(vec3(11.1, float(i) * 7.1, 1.1)) * 2.0 - 1.0
   );
   p = p0 + o * 0.08;
   vec2 pi = floor(p * s + 0.5) / s;
   float mpiy = mod(pi.y * s, 2.0);
   if (mpiy == 0.0) {
     pi.x = floor(p.x * s + 1.0) / s - 0.5 / s;
   }
   float ids = float(i) * 1.0;
   vec2 off = vec2(rand(vec3(pi, ids)), rand(vec3(pi, 1.7 + ids)));
   vec2 pos = pi + off * (1.0/s/3.0);
   float d = length(p - pos);
   float rvp = rand(vec3(pi + o, 0.0));
   float thresh = 0.2 - pos.x * pos.x * sin(u_time);
   if (rvp > thresh) {
     vec2 x = p - pos;
     x = rotate(x, off.x * 10.0);
     float sdf = sdRoundedBox(x, vec2(0.3 / s, 0.03 / s), vec4(0.005 / s));
     c += sdf < 0.0 ? 0.5 : 0.0;
   }
 }
 gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0) * c;
}


