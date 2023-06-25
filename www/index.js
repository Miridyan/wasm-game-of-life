import { Universe, greet } from 'wasm-game-of-life';

const pre = document.getElementById('game-of-life-canvas');
const universe = Universe.new()
const sleep = ms => new Promise(r => setTimeout(r, ms));

const renderLoop = () => {
    sleep(100).then(() => {
        pre.textContent = universe.render();
        universe.tick();

        requestAnimationFrame(renderLoop);
    })
}

greet()
requestAnimationFrame(renderLoop);
