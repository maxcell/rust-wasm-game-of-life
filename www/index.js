// Following this example: https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
import init, {Universe, Cell} from './pkg/wasm_game_of_life.js';

const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';
const INACTIVE_COLOR = '#FFFFFF';
const ACTIVE_COLOR = '#000000';


async function run() {
  // init is necessary to load in the WASM file
  const {memory} = await init()

  // Then we can use everything that was loaded in through WASM
  const universe = Universe.new()
  const width = universe.width()
  const height = universe.height()

  const canvas = document.getElementById("game-of-life-canvas")
  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  const ctx = canvas.getContext('2d')

  const getIndex = (row, column) => {
    return row * width + column;
  }

  const drawGrid = () => {
    ctx.beginPath()
    ctx.strokeStyle = GRID_COLOR;

    for(let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1)
    }

    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1)
    }

    ctx.stroke()
  }

  const drawCells = () => {
    const cellsPtr = universe.cells()
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath()

    for(let row = 0; row < height; row++) {
      for(let col = 0; col < width; col++) {
        const index = getIndex(row, col)

        ctx.fillStyle = cells[index] == Cell.Inactive ? INACTIVE_COLOR : ACTIVE_COLOR;
        ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE)
      }
    }

    ctx.stroke()
  }

  const renderLoop = () => {
    universe.tick()
    drawGrid();
    drawCells();
    requestAnimationFrame(renderLoop)
  }

  drawGrid()
  drawCells()
  requestAnimationFrame(renderLoop)
}

run()

