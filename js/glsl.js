// don't know how build system works, ugh
THREE = Reveal.THREE;

const canvas = document.getElementById('three-canvas');
const scene = new THREE.Scene();
const camera = new THREE.OrthographicCamera(-0.5, 0.5, 0.5, -0.5, 0, 1);

const renderer = new THREE.WebGLRenderer({ canvas: canvas, alpha: true });

const smaterial = new THREE.ShaderMaterial({
  uniforms: {
    uTime: { value: 0.0 },
    uResolution: { value: new THREE.Vector2(canvas.width, canvas.height) },
  },
  vertexShader: `
    void main() {
      gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
    }
  `,
  fragmentShader: `
    uniform float uTime;
    uniform vec3 uResolution;

// white noise (noise from [0, 1] uncorrelated with neighbor pixels)
float rand(vec3 x) { 
    return fract(sin(dot(x, vec3(11.1191, 78.233, 36.15861))) * 43758.5453);
}
float safe_rand(vec3 x) {
    return rand(floor(x * 408.12935));
}

void main()
{
    vec3 p = gl_FragCoord.xyz / uResolution.xyy;
    if (p.x < 0.5) {
        p.x += uTime * 0.01;
    }
    float w;
    if (p.y < 0.5) {
        w = safe_rand(p);
    } else {
        w = rand(p);
    }
    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0) * w;
}

    `
});
const sprite = new THREE.Sprite(smaterial);
scene.add(sprite);
const timer = new THREE.Timer();
timer.connect(document);
console.log(smaterial.uniforms.uResolution);

function animate(dt) {
    requestAnimationFrame(animate);
    timer.update();
    smaterial.uniforms.uTime.value = timer.getElapsed();
    renderer.render(scene, camera);
}
animate();
