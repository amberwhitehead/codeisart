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
    void main() {
      vec3 p = gl_FragCoord.xyz / uResolution.xyz;
      float r = abs(sin(uTime));
      gl_FragColor = vec4(0.0, 0.2, p.y > 0.50 ? 1.0 : 0.0, 1.0);
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
