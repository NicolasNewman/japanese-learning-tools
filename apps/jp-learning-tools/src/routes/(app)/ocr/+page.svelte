<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  type Region = {
    x: number;
    y: number;
    width: number;
    height: number;
  };

  let region = $state({ x: 0, y: 0, width: 0, height: 0 });

  onMount(() => {
    let unMountFunction: UnlistenFn | null = null;

    listen<Region>("region-selected", (event) => {
      region = event.payload;
      console.log("Received region:", region);
    }).then((fn) => {
      unMountFunction = fn;
    });

    return () => {
      if (unMountFunction) {
        unMountFunction();
      }
    };
  });

  async function startRegionCapture() {
    try {
      await invoke("start_region_select");
    } catch (error) {
      console.error("Failed to start region selector:", error);
    }
  }
</script>

<div class="flex gap-4">
  <button class="btn btn-primary" onclick={startRegionCapture}>
    Select Region for OCR
  </button>
  <button
    class="btn btn-primary"
    onclick={async () => {
      try {
        const text = await invoke<string[]>("capture", region);
        console.log("Captured text:", text);
      } catch (error) {
        console.error("Failed to capture region:", error);
      }
    }}
  >
    Capture</button
  >
</div>
<div>
  <div>x: {region.x}</div>
  <div>y: {region.y}</div>
  <div>width: {region.width}</div>
  <div>height: {region.height}</div>
</div>
