#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform float u_time;


// white noise (noise from [0, 1] uncorrelated with neighbor pixels)
float rand(vec3 x) { 
    return fract(sin(dot(x, vec3(11.1191, 78.233, 36.15861))) * 43758.5453);
}
float safe_rand(vec3 x) {
    return rand(floor(x * 408.12935));
}

void main()
{
    vec3 p = gl_FragCoord.xyz / u_resolution.xyy;
    if (p.x < 0.5) {
        p.x += u_time * 0.01;
    }
    float w;
    if (p.y < 0.5) {
        w = safe_rand(p);
    } else {
        w = rand(p);
    }
    float n = w;
    gl_FragColor = vec4(1.0, 1.0, 1.0, 0.0) * n;
}
