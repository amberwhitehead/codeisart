import GlslCanvas from 'glslCanvas';
import prefix from './prefix.glsl?raw';
import noise from './noise.glsl?raw';
import perlin from './perlin.glsl?raw';

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
            sandbox.load(prefix + shaderSrc);
        });
        console.log(el);
        const buttonRow = document.getElementById('buttons');
        buttonRow.appendChild(el);
    }
    addButton('1', noise);
    addButton('2', perlin);
});
