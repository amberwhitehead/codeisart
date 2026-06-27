// don't know how build system works, ugh
THREE = Reveal.THREE;

document.querySelectorAll(".three-canvas").forEach(async (canvas) => {
    let fragtext = await (await fetch(canvas.dataset.frag)).text();
    console.log(canvas, fragtext);
    const scene = new THREE.Scene();
    const camera = new THREE.OrthographicCamera(-0.5, 0.5, 0.5, -0.5, 0, 1);

    const renderer = new THREE.WebGLRenderer({ canvas: canvas, alpha: true });

    const smaterial = new THREE.ShaderMaterial({
        uniforms: {
            u_time: { value: 0.0 },
            u_resolution: { value: new THREE.Vector2(canvas.width, canvas.height) },
        },
        vertexShader: `void main() { gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0); }`,
        fragmentShader: `
uniform float u_time;
uniform vec3 u_resolution;

${fragtext}
`});
    const sprite = new THREE.Sprite(smaterial);
    scene.add(sprite);
    const timer = new THREE.Timer();
    timer.connect(document);
    console.log(smaterial.uniforms.u_resolution);

    function animate(dt) {
        requestAnimationFrame(animate);
        timer.update();
        smaterial.uniforms.u_time.value = timer.getElapsed();
        renderer.render(scene, camera);
    }
    animate();
});
