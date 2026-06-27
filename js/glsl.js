// don't know how build system works, ugh
THREE = Reveal.THREE;

const canvas = document.getElementById('three-canvas');
const scene = new THREE.Scene();
const camera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0, 1);

const renderer = new THREE.WebGLRenderer({ canvas: canvas, alpha: true });

const geometry = new THREE.PlaneGeometry(2, 2);
const smaterial = new THREE.ShaderMaterial({
  uniforms: {
    uTime: { value: 0.0 }
  },
  vertexShader: `
    void main() {
      gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
    }
  `,
  fragmentShader: `
    uniform float uTime;
    void main() {
      vec3 p = gl_FragCoord.xyz;
      float r = abs(sin(uTime));
      gl_FragColor = vec4(r, 0.2, p.y > 125.5 ? 0.5 : 0.0, 1.0);
    }
  `
});
const sprite = new THREE.Mesh(geometry, smaterial);
scene.add(sprite);
const timer = new THREE.Timer();
timer.connect(document);

function animate(dt) {
    requestAnimationFrame(animate);
    timer.update();
    smaterial.uniforms.uTime.value = timer.getElapsed();
    renderer.render(scene, camera);
}
animate();
