import { Cell, Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 8;
const GRID_COLOR = "#928374";
const DEAD_COLOR = "#282828";
const ALIVE_COLOR = "#FBF1C7";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const getIndex = (row, col) => row * width + col;

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;

const ctx = canvas.getContext('2d');

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        const x = i * (CELL_SIZE + 1) + 1;
        ctx.moveTo(x, 0);
        ctx.lineTo(x, (CELL_SIZE + 1) * height + 1);
    }

    for (let j = 0; j <= height; j++) {
        const y = j * (CELL_SIZE + 1) + 1;
        ctx.moveTo(0, y);
        ctx.lineTo(width * (CELL_SIZE + 1) + 1, y);
    }

    ctx.stroke();
}

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] == Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

            // (x, y, w, h)
            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
