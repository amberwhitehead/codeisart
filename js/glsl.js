// don't know how build system works, ugh
THREE = Reveal.THREE;


const canvas = document.getElementById('three-canvas');
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer({ canvas: canvas, alpha: true });
renderer.setSize(window.innerWidth, window.innerHeight);

const geometry = new THREE.BoxGeometry();
const material = new THREE.MeshBasicMaterial({ color: 0x00ff00, wireframe: true });

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
const cube = new THREE.Mesh(geometry, smaterial);
scene.add(cube);
camera.position.z = 5;
const timer = new THREE.Timer();
timer.connect(document);

// Reveal.initialize({
//     hash: true,
//     // ... add other options here
// });

function animate(dt) {
    requestAnimationFrame(animate);
    timer.update();
    cube.rotation.x += 0.01;
    cube.rotation.y += 0.01;
    smaterial.uniforms.uTime.value = timer.getElapsed();
    renderer.render(scene, camera);
}
animate();

// Reveal.addEventListener('slidechanged', (event) => {
//     const currentSlide = event.currentSlide;
//     // Example: Move the camera or change cube color when slide 2 is active
//     if (event.indexh === 1) {
//         cube.material.color.setHex(0xff0000);
//     } else {
//         cube.material.color.setHex(0x00ff00);
//     }
// });
