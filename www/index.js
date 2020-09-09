import { Universe } from "./pkg/wasm_game_of_life.js";

const pre = document.getElementById("game-of-life-canvas")
const universe = Universe.new(64, 64)
const renderLoop = () => {
  pre.textContent = universe.render();
  universe.tick()
  requestAnimationFrame(renderLoop)
}

requestAnimationFrame(renderLoop)