<script lang="ts">
    import { emit } from "@tauri-apps/api/event";
  // import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  let isDrawing = false;
  let startX = $state(0);
  let startY = $state(0);
  let currentX = $state(0);
  let currentY = $state(0);

  let x = $derived.by(() => Math.min(startX, currentX));
  let y = $derived.by(() => Math.min(startY, currentY));
  let width = $derived.by(() => Math.abs(currentX - startX));
  let height = $derived.by(() => Math.abs(currentY - startY));

  function handleMouseDown(e: MouseEvent) {
    if (e.button !== 0) return; // Only respond to left-click
    console.log("handleMouseDown", e.clientX, e.clientY);
    isDrawing = true;
    startX = e.clientX;
    startY = e.clientY;
    currentX = startX;
    currentY = startY;
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDrawing) return;
    currentX = e.clientX;
    currentY = e.clientY;
  }

  async function handleMouseUp(e: MouseEvent) {
    if (!isDrawing) return;
    isDrawing = false;
    //     const endX = e.clientX;
    //     const endY = e.clientY;
    //     // Calculate region
    //     const x = Math.min(startX, endX);
    //     const y = Math.min(startY, endY);
    //     const width = Math.abs(endX - startX);
    //     const height = Math.abs(endY - startY);
    // Close this window
    // setRegion({x, y, width, height});
    await emit("region-selected", { x, y, width, height });

    const window = getCurrentWindow();
    await window.close();
    // await window.hide();
    // // Perform OCR on selected region
    // try {
    //   const text = await invoke<string>("capture", { x, y, width, height });
    //   console.log("Recognized text:", text);
    //   // You can emit an event or store the result
    // } catch (error) {
    //   console.error("OCR failed:", error);
    // }

  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      getCurrentWindow().close();
    }
  }
</script>

<svelte:window on:keydown={handleKeyDown} />

<!-- <canvas
  bind:this={canvas}
  on:mousedown={handleMouseDown}
  on:mousemove={handleMouseMove}
  on:mouseup={handleMouseUp}
></canvas> -->

<!-- <Stage
  width={window.innerWidth}
  height={window.innerHeight}
  onmousedown={handleMouseDown}
  onmousemove={handleMouseMove}
  onmouseup={handleMouseUp}
>
  <Layer>
    <Rect
      x={0}
      y={0}
      width={window.innerWidth}
      height={window.innerHeight}
      fill="rgba(0, 0, 0, 0.3)"
    />
    <Rect bind:x bind:y {width} {height} stroke="#00ff00" strokeWidth={2} />
  </Layer>
</Stage> -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="canvas"
  onmousedown={handleMouseDown}
  onmousemove={handleMouseMove}
  onmouseup={handleMouseUp}
>
  <span
    id="region"
    style={`top: ${y}px; left: ${x}px; width: ${width}px; height: ${height}px;`}
  ></span>
</div>

<div class="instructions">
  Click and drag to select a region. Press ESC to cancel.
</div>

<style>
  :global(body) {
    background-color: transparent !important;
    background: transparent !important;
    /* overflow: hidden; */
  }

  #region {
    position: fixed;
    /* background: rgba(0, 255, 0, 0.3); */
    border: 2px solid #00ff00;
  }
  .canvas {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    cursor: crosshair;
    background: rgb(0 0 0 / 30%);
    /* background: transparent; */
  }

  .instructions {
    position: fixed;
    top: 20px;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(0, 0, 0, 0.8);
    color: white;
    padding: 10px 20px;
    border-radius: 5px;
    font-size: 14px;
    pointer-events: none;
    z-index: 1000;
  }
</style>
