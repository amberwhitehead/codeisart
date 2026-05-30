import GlslCanvas from 'glslCanvas';
import prefix from './prefix.glsl?raw';
import noise from './noise.glsl?raw';
import perlin from './perlin.glsl?raw';
import fbm from './fbm.glsl?raw';
import turbulence from './turbulence.glsl?raw';
import nested from './nested.glsl?raw';
import dither1 from './dither1.glsl?raw';
import dither2 from './dither2.glsl?raw';

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
    addButton('3', fbm);
    addButton('4', turbulence);
    addButton('5', nested);
    addButton('6', dither1);
    addButton('7', dither2);
});
