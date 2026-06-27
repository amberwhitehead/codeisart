// don't know how build system works, ugh
THREE = Reveal.THREE;

const canvas = document.getElementById('three-canvas');
const scene = new THREE.Scene();
const camera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0, 1);

const renderer = new THREE.WebGLRenderer({ canvas: canvas, alpha: true });
renderer.setSize(window.innerWidth, window.innerHeight);

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
      float r = abs(sin(uTime));
      gl_FragColor = vec4(r, 0.2, 0.5, 1.0);
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
        // cube.rotation.x += 0.01;
        // cube.rotation.y += 0.01;
    smaterial.uniforms.uTime.value = timer.getElapsed();
    renderer.render(scene, camera);
}
animate();
