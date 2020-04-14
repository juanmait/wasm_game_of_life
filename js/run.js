const CELL_SIZE = 15; // px
const GRID_COLOR = "#EEEEEE";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

export const run = async ({ Universe, Cell }, memory) => {
  const universe = Universe.new();
  const width = universe.width();
  const height = universe.height();

  const canvas = document.getElementById("game-of-life-canvas");
  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  const ctx = canvas.getContext("2d");

  const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
  };

  /**
   * Given an index and Uint8Array, determine whether
   * the nth bit is set
   * @param {number} n index
   * @param {array} arr bits
   */
  const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << n % 8;
    return (arr[byte] & mask) === mask;
  };

  const getIndex = (row, column) => {
    return row * width + column;
  };

  const drawCells = () => {
    const cellsPtr = universe.cells();
    // constructing a Uint8Array from Wasm memory is the same as before,
    // except that the length of the array is not width * height anymore,
    // but width * height / 8 since we have a cell per bit rather than
    // per byte
    const cells = new Uint8Array(memory.buffer, cellsPtr, (width * height) / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR;

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

  canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
  });

  let animationId = null;

  const isPaused = () => {
    return animationId === null;
  };

  const playPauseButton = document.getElementById("play-pause");

  playPauseButton.textContent = "▶";

  const play = () => {
    playPauseButton.textContent = "⏸";
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

  const renderLoop = () => {
    drawGrid();
    drawCells();
    universe.tick();
    // debugger;
    animationId = requestAnimationFrame(renderLoop);
  };
};
