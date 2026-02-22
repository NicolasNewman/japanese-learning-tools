<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
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
  let captureEnabled = $state(false);
  let capturedText = $state<string[]>([]);

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

  async function startCapture() {
    captureEnabled = true;
    const cb = async () => {
      if (!captureEnabled) return;
      try {
        const text = await invoke<string[]>("capture", region);
        console.log("Captured text:", text);
        capturedText = text;
        setTimeout(cb, 1000);
      } catch (error) {
        console.error("Failed to capture region:", error);
      }
    };

    setTimeout(cb, 1000);
  }

  function stopCapture() {
    captureEnabled = false;
  }
</script>

<main class="container p-4 flex flex-col gap-4">
  <div class="flex gap-4">
    <Button class="btn" onclick={startRegionCapture}
      >Select Capture Region</Button
    >
    {#if region.width > 0 && region.height > 0}
      {#if !captureEnabled}
        <Button onclick={startCapture}>Start</Button>
      {:else}
        <Button variant="destructive" onclick={stopCapture}>Stop</Button>
      {/if}
    {/if}
  </div>
  {#if region.width > 0 && region.height > 0}
    <div>
      Selected Region: x={region.x}, y={region.y}, width={region.width}, height={region.height}
    </div>
  {/if}
  {#if capturedText.length > 0}
    <div>
      <h2>Captured Text:</h2>
      <ul>
        {#each capturedText as text}
          <li>{text}</li>
        {/each}
      </ul>
    </div>
  {/if}
</main>
