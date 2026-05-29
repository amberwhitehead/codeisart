import GlslCanvas from 'glslCanvas';
import frag from './frag.glsl?raw';

document.addEventListener("DOMContentLoaded", (event) => {
    console.log('Hello shaders', GlslCanvas);
    const canvas = document.getElementById("glslCanvas");
    const sandbox = new GlslCanvas(canvas);
    sandbox.load(frag);
    document.body.appendChild(canvas);
});
