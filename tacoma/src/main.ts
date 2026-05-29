import GlslCanvas from 'glslCanvas';
import frag from './noise.glsl?raw';

function handleButton(evt) {
    console.log(evt);
}


document.addEventListener("DOMContentLoaded", (event) => {
    for (const el of document.getElementsByTagName('button')) {
        console.log(el);
        el.addEventListener('click', handleButton);
    }
    const canvas = document.getElementById("glslCanvas");
    const sandbox = new GlslCanvas(canvas);
    function addButton(name, shaderSrc) {
        const el = document.createElement('button');
        el.innerText = name;
        el.addEventListener('click', () => {
            sandbox.load(shaderSrc);
        });
        console.log(el);
        document.body.appendChild(el);
    }
    addButton('1', frag);
});
