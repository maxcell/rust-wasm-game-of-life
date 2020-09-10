// Following this example: https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
import init, {Universe} from './pkg/wasm_game_of_life.js';

async function run() {
  // init is necessary to load in the WASM file
  await init()

  // Then we can use everything that was loaded in through WASM
  const universe = Universe.new()
  const pre = document.getElementById("game-of-life-canvas")
  const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick()
    requestAnimationFrame(renderLoop)
  }

  requestAnimationFrame(renderLoop)
}

run()

