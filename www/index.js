import { Universe, Cell } from "game-of-life-wasm-haochun";
import { memory } from "game-of-life-wasm-haochun/game_of_life_wasm_haochun_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const universe = Universe.new();
const height = universe.width(); 
const width = universe.height();

let numberOfTicks = 1;
const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of the latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(min, this.frames[i]);
            max = Math.max(max, this.frames[i]);
        }
        let mean = sum / this.frames.length;
        this.fps.textContent = `
Frames Per Second: ${Math.round(mean)} (${Math.round(min)}~${Math.round(max)})
`.trim();
    }
};

const getIndex = (row, column) => {
  return row * width + column;
};

const isAlive = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) == mask;
};

var slider = document.getElementById("tick-range");
slider.oninput = function() {
    numberOfTicks = this.value;
}

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(
        Math.floor(canvasTop / (CELL_SIZE + 1)),
        height - 1
    );
    const col = Math.min(
        Math.floor(canvasLeft / (CELL_SIZE + 1)),
        width - 1
    );

    if (event.metaKey) {
        universe.add_glider(row, col);
    } else if (event.shiftKey) {
        // May have uncaught Error: recursive use of object.
        universe.add_pulsar(row, col);
    } else {
        universe.toggle_cell(row, col);
    }
    drawGrid();
    drawCells();
});


let animationId = null;

const renderLoop = () => {

    fps.render();
    for (let i = 0; i < numberOfTicks; i++) {
        universe.tick();
    }

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
}

const restartButton = document.getElementById("restart");
restartButton.addEventListener("click", event => {
    universe.reset(0.5);
});

const purgeButton = document.getElementById("purge");
purgeButton.addEventListener("click", event => {
    universe.purge();
});


// Play And Pause buttons
const isPaused = () => {
    return animationId === null;
};
const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = String.fromCharCode('0x23F8');
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});
// ====================

// TODO: Use WebGL Renderer
const ctx = canvas.getContext('2d');
const drawGrid = () => {
    ctx.beginPath();
    ctx.stroke_style = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
    ctx.stroke();
};

const drawCells = () => {
    const cellsptr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsptr, width * height / 8);

    ctx.beginPath();
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = isAlive(idx, cells)
                ? ALIVE_COLOR
                : DEAD_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    ctx.stroke();
};

drawGrid();
drawCells();
play();
