// don't know how build system works, ugh
THREE = Reveal.THREE;


const canvas = document.getElementById('three-canvas');
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer({ canvas: canvas, alpha: true });
renderer.setSize(window.innerWidth, window.innerHeight);

const geometry = new THREE.BoxGeometry();
const material = new THREE.MeshBasicMaterial({ color: 0x00ff00, wireframe: true });
const cube = new THREE.Mesh(geometry, material);
scene.add(cube);
camera.position.z = 5;

// Reveal.initialize({
//     hash: true,
//     // ... add other options here
// });

function animate() {
    requestAnimationFrame(animate);
    cube.rotation.x += 0.01;
    cube.rotation.y += 0.01;
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
