<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Input } from "$lib/components/ui/input";
  import { Spinner } from "$lib/components/ui/spinner";
  import IconFile from "@lucide/svelte/icons/file";
  import type { UnlistenFn } from "@tauri-apps/api/event";
  import { resourceDir, sep } from "@tauri-apps/api/path";
  import { onDestroy, onMount } from "svelte";
  import {
    type MpvObservableProperty,
    type MpvConfig,
    init,
    observeProperties,
    command,
    setProperty,
    getProperty,
    destroy,
  } from "tauri-plugin-libmpv-api";
  // TODO: switch player
  // Properties to observe
  // Tip: The optional third element, 'none', signals to TypeScript that the property's value may be null
  // (e.g., when a file is not loaded), ensuring type safety in the callback function.
  const OBSERVED_PROPERTIES = [
    ["pause", "flag"],
    ["time-pos", "double", "none"],
    ["duration", "double", "none"],
    ["filename", "string", "none"],
  ] as const satisfies MpvObservableProperty[];

  let isRunning: boolean = $state(false);
  let isLoading: boolean = $state(false);
  let mediaFile: string | null = $state(null);

  let unlisten: null | UnlistenFn = $state(null);

  const startMPV = async () => {
    isLoading = true;
    try {
      const resourcePath = await resourceDir();
      const mpvPath = `${resourcePath}${sep()}resources${sep()}mpv${sep()}`;

      // mpv configuration
      const mpvConfig: MpvConfig = {
        initialOptions: {
          alang: "ja,jp,jpn,japanese,en,eng,english,English,enUS,en-US",
          slang: "ja,jp,jpn,japanese,en,eng,english,English,enUS,en-US",
          // TODO: make these configurable
          "screenshot-directory": "~/Pictures/Screenshots/",
          "screenshot-template": "%F_%wHh%wMm%wSs%wTms",
          "sub-auto": "fuzzy",
          "subs-with-matching-audio": "yes",
          "screenshot-format": "jpg",
          "screenshot-jpeg-quality": 90,
          "screenshot-high-bit-depth": "yes",
          "sub-font-size": 40,
          vo: "gpu-next",
          hwdec: "auto-safe",
          "keep-open": "yes",
          "force-window": "yes",
          "load-scripts": "no",
          include: `${mpvPath}input.conf`,
          "osd-fonts-dir": `${mpvPath}fonts${sep()}`,
          "osd-font": "Material Design Iconic Font",
          osc: "no",
        },
        observedProperties: OBSERVED_PROPERTIES,
      };

      await init(mpvConfig);
      await command("load-script", [
        `${mpvPath}scripts${sep()}mpvacious${sep()}`,
      ]);
      await command("load-script", [
        `${mpvPath}scripts${sep()}ModernX${sep()}`,
      ]);

      if (mediaFile) {
        await command("loadfile", [mediaFile]);
      }

      unlisten = await observeProperties(
        OBSERVED_PROPERTIES,
        ({ name, data }) => {
          switch (name) {
            case "pause":
              console.log("Playback paused state:", data);
              break;
            case "time-pos":
              console.log("Current time position:", data);
              break;
            case "duration":
              console.log("Duration:", data);
              break;
            case "filename":
              console.log("Current playing file:", data);
              break;
          }
        },
      );

      isRunning = true;
      isLoading = false;
    } catch (error) {
      console.error("mpv initialization failed:", error);
      isRunning = false;
      isLoading = false;
    }
  };

  const stopMPV = async () => {
    if (isRunning) {
      if (unlisten) {
        unlisten();
      }
      await destroy();
      isRunning = false;
    }
  };

  // onDestroy(async () => {
  //   console.log("Cleaning up mpv resources...");
  //   if (unlisten) {
  //     unlisten();
  //   }
  //   await destroy();
  // });
</script>

<div class="p-4">
  <div class="grid grid-cols-2">
    <div>
      <h2 class="text-2xl font-bold mb-4">mpv Player Control</h2>
      <p class="text-sm text-muted-foreground">
        Start and stop the mpv player, and select video files to play.
      </p>
      <h2 class="text-2xl font-bold mb-4">mpvacious</h2>
    </div>
    <div>B</div>
  </div>
  <div class="flex gap-4 mt-4 items-center">
    <Button
      placeholder="Select video file..."
      disabled={isLoading || isRunning}
      onclick={async () => {
        mediaFile = await open({
          multiple: false,
          directory: false,
          filters: [
            {
              name: "Video File",
              extensions: ["mp4", "mkv", "avi", "mov", "flv"],
            },
          ],
        });
        if (isRunning && mediaFile) {
          await command("loadfile", [mediaFile]);
        }
      }}
      ><IconFile /> Select Video
    </Button>
    {#if !isRunning}
      <Button onclick={startMPV} disabled={isRunning || isLoading}>
        {#if isLoading}
          <Spinner />{/if}
        Start mpv
      </Button>
    {:else}
      <Button
        onclick={stopMPV}
        disabled={!isRunning || isLoading}
        variant="destructive"
      >
        {#if isLoading}
          <Spinner />{/if}
        Stop mpv
      </Button>
    {/if}
    <div class="text-sm text-muted-foreground">
      {#if mediaFile}
        Selected file: {mediaFile.split("/").pop()}
      {:else}
        No file selected
      {/if}
    </div>
  </div>
</div>
