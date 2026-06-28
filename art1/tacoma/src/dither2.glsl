
// Translating https://www.shadertoy.com/view/3ssGWj

#define MAX_STEPS 100
#define MAX_DIST 1000.
#define SURF_DIST .001

float sdTorus( vec3 p, vec2 t ) {
  vec2 q = vec2(length(p.xz) - t.x, p.y);
  return length(q) - t.y;
}

mat2 Rot(float a) {
    float s = sin(a);
    float c = cos(a);
    return mat2(c, -s, s, c);
}

float smin(float a, float b, float k) {
    float h = clamp( 0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}

vec3 R(vec2 uv, vec3 p, vec3 l, float z) {
    vec3 f = normalize(l - p),
        r = normalize(cross(vec3(0, 1, 0), f)),
        u = cross(f, r),
        c = p + f * z,
        i = c + uv.x * r + uv.y * u,
        d = normalize(i - p);
    return d;
}

float GetDist(vec3 p) {
    float t = u_time;
    // ground plane
    float pd = p.y;
    // torus
    float y = 0.0;
    vec3 tp = p;
    // translate
    tp -= vec3(0.0, 1.1, 0.0);
    // scale
    // flip torus on its side
    tp = tp.xzy;
    float scale = sdTorus(tp, vec2(1.0, 0.6));
    float d = pd;
    d = min(d, scale);
    return d;
}

vec3 GetNormal(vec3 p) {
	float d = GetDist(p);
    vec2 e = vec2(0.001, 0);
    vec3 n = d - vec3(
        GetDist(p - e.xyy),
        GetDist(p - e.yxy),
        GetDist(p - e.yyx));  
    return normalize(n);
}

float RayMarch(vec3 ro, vec3 rd) {
	float dO=0.0;
    for(int i = 0; i < MAX_STEPS; i++) {
    	vec3 p = ro + rd * dO;
        float dS = abs(GetDist(p));
        dO += dS;
        if(dO > MAX_DIST || dS < SURF_DIST) {
            break;
        }
    }
    return dO;
}

float GetLight(vec3 p) {
    vec3 lightPos = vec3(3, 5, 4);
    vec3 l = normalize(lightPos - p);
    vec3 n = GetNormal(p);
    
    float dif = clamp(dot(n, l) * 0.5 + 0.5, 0.0, 1.0);
    float d = RayMarch(p + n * SURF_DIST * 2.0, l);
    if(p.y < .01 && d < length(lightPos - p)) {
        dif *= .5;
    }
    return dif;
}

void main()
{
  vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / u_resolution.y;
  vec3 col = vec3(0);
  vec3 ro = vec3(0, 3, -3);
  ro.yz *= Rot(0.0);
  ro.xz *= Rot(u_time * 0.8);
  vec3 rd = R(uv, ro, vec3(0, 0, 0), .7);
  float d = RayMarch(ro, rd);
  if(d < MAX_DIST) {
    vec3 p = ro + rd * d;
    float dif = GetLight(p);
    col = vec3(dif);
  }

  gl_FragColor = vec4(col, 1.0);
}
