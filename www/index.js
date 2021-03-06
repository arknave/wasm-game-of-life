import { Cell, Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 8;
const GRID_COLOR = "#928374";
const DEAD_COLOR = "#282828";
const ALIVE_COLOR = "#FBF1C7";

const universe = Universe.new();
universe.random_cells();
const width = universe.width();
const height = universe.height();

const getIndex = (row, col) => row * width + col;

// drawing

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

const accessCell = (cells, idx) => {
    const cellIdx = idx >> 3;
    const mask = 1 << (idx & 0x7);
    return (cells[cellIdx] & mask) > 0;
}

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = accessCell(cells, idx) ? DEAD_COLOR : ALIVE_COLOR;

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

const draw = () => {
    drawGrid();
    drawCells();
}

// canvas interaction

const clamp = (x, lo, hi) => {
    return Math.min(Math.max(lo, x), hi);
}

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = clamp(Math.floor(canvasTop / (CELL_SIZE + 1)), 0, height - 1);
    const col = clamp(Math.floor(canvasLeft / (CELL_SIZE + 1)), 0, width - 1);

    if (event.metaKey || event.ctrlKey) {
        universe.add_spaceship(row, col);
    } else if (event.shiftKey) {
        universe.add_pulsar(row, col);
    } else {
        universe.toggle_cell(row, col);
    }

    draw();
});

// User controls

const playPauseButton = document.getElementById("play-pause");

let animationId = null;

const isPaused = () => {
    return animationId === null;
}

const play = () => {
    playPauseButton.textContent = "Pause";
    renderLoop();
}

const pause = () => {
    playPauseButton.textContent = "Play";
    cancelAnimationFrame(animationId);
    animationId = null;
}

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

const randomButton = document.getElementById("random-board");
randomButton.addEventListener("click", event => {
    universe.random_cells();
    draw();
});

const resetButton = document.getElementById("clear-board");
resetButton.addEventListener("click", event => {
    universe.reset_cells();
    draw();
});

const frameSkipSlider = document.getElementById("speed");

const getFrameSkip = () => {
    return frameSkipSlider.max - 1 - frameSkipSlider.value;
}
let frameSkip = getFrameSkip();

frameSkipSlider.addEventListener("input", event => {
    frameSkip = getFrameSkip();
});

const helpButton = document.getElementById("help");
helpButton.addEventListener("click", event => {
    alert("Welcome! The grid is toroidal. Click on a cell to toggle. CMD-click to insert a glider. Shift-click to insert a pulsar.");
});

let tickCount = 0;
const renderLoop = () => {
    tickCount++;
    if (tickCount >= frameSkip) {
        universe.tick();
        tickCount = 0;
    }

    draw();

    animationId = requestAnimationFrame(renderLoop);
}

play();
