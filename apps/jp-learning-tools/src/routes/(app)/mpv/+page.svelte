<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Spinner } from "$lib/components/ui/spinner";
  import IconFile from "@lucide/svelte/icons/file";
  import { resourceDir, sep } from "@tauri-apps/api/path";
  import MPVacious from "$lib/components/mpv/mpvacious-instructions.svelte";
  import { alertState } from "../../../stores/alertState.svelte";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import {
    type MpvObservableProperty,
    type MpvConfig,
    init,
    observeProperties,
    command,
    destroy,
    listenEvents,
  } from "tauri-plugin-libmpv-api";
  import HowTo from "$lib/components/mpv/how-to.svelte";
  import {
    toMpvScriptOpt,
    get,
    addToWatchHistory,
    updateWatchHistory,
  } from "$lib/persistence/mpvStore";
  import Control from "$lib/components/mpv/control.svelte";
  const OBSERVED_PROPERTIES = [
    ["pause", "flag"],
    ["time-pos", "double", "none"],
    ["duration", "double", "none"],
    ["filename", "string", "none"],
    ["sub-delay", "double", "none"],
  ] as const satisfies MpvObservableProperty[];
  import { mpvState } from "../../../stores/mpvState.svelte";
  import History from "$lib/components/mpv/history.svelte";

  let activeTab: "home" | "how-to" = $state("home");
  let subOffset: number = 0;
  let timestamp: number = 0;

  const startMPV = async () => {
    mpvState.isLoading = true;
    try {
      const resourcePath = await resourceDir();
      const mpvPath = `${resourcePath}${sep()}resources${sep()}mpv${sep()}`;
      console.log(toMpvScriptOpt((await get("script-opts")) ?? []));
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
          "script-opts": toMpvScriptOpt((await get("script-opts")) ?? []),
          // "input-conf": `${mpvPath}input.conf`,
          // "script-opts":
          // "subs2srs-autoclip_method=clipboard,subs2srs-autoclip=yes,subs2srs-deck_name=Active::Japanese::subs2srs,subs2srs-note_tag=subs2srs kanji-first",
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
      await command("load-config-file", [
        `${mpvPath}script-opts${sep()}subs2srs.conf`,
      ]);

      await command("load-script", [
        `${mpvPath}scripts${sep()}ModernZ${sep()}`,
      ]);

      await command("load-input-conf", [`${mpvPath}input.conf`]);

      if (mpvState.mediaFile) {
        await command("loadfile", [mpvState.mediaFile]);
        // watch history id may have been set if the user clicked on a history item
        if (mpvState.watchHistoryId === null) {
          addToWatchHistory(mpvState.mediaFile ?? "").then((id) => {
            mpvState.watchHistoryId = id;
          });
        }
      }
      mpvState.unlisten = await observeProperties(
        OBSERVED_PROPERTIES,
        async ({ name, data }) => {
          switch (name) {
            case "pause":
              console.log("Playback paused state:", data);
              break;
            case "time-pos":
              if (data) {
                timestamp = data;
              }
              break;
            case "duration":
              console.log("Duration:", data);
              break;
            case "sub-delay":
              console.log("Subtitle delay:", data);
              if (data) {
                subOffset = data;
              }
              break;
            case "filename":
              console.log("Current playing file:", data);
              break;
          }
        },
      );

      mpvState.unlistenEvents = await listenEvents(async (e) => {
        switch (e.event) {
          case "file-loaded":
            console.log("File loaded:");
            if (mpvState.restorePoint) {
              await command("seek", [
                Math.round(mpvState.restorePoint.timestamp),
                "absolute+keyframes",
              ]);
              await command("set", [
                "sub-delay",
                mpvState.restorePoint.subOffset,
              ]);
              mpvState.restorePoint = null;
            }

            break;
        }
      });

      mpvState.isRunning = true;
      mpvState.isLoading = false;
    } catch (error) {
      alertState.alert = {
        alertTitle: "Failed to initialize MPV",
        alertMessage: `${error}`,
        alertType: "error",
      };

      console.error("mpv initialization failed:", error);
      await stopMPV(true);
    }
  };

  const stopMPV = async (failure: boolean = false) => {
    if (mpvState.isRunning || mpvState.isLoading) {
      if (mpvState.unlisten) {
        mpvState.unlisten();
        mpvState.unlisten = null;
      }
      if (mpvState.unlistenEvents) {
        mpvState.unlistenEvents();
        mpvState.unlistenEvents = null;
      }
      await destroy();
      mpvState.isRunning = false;
      mpvState.isLoading = false;
      if (mpvState.watchHistoryId !== null && !failure) {
        updateWatchHistory(mpvState.watchHistoryId, {
          timestamp,
          subOffset,
        });
        mpvState.watchHistoryId = null;
      }
    }
  };
</script>

<div class="flex flex-col p-4 h-[calc(100vh-30px)] justify-between">
  <div class="overflow-y-scroll mb-2">
    <Tabs.Root bind:value={activeTab}>
      <Tabs.List class="mb-2">
        <Tabs.Trigger value="home">Home</Tabs.Trigger>
        <Tabs.Trigger value="how-to">How-to</Tabs.Trigger>
        <Tabs.Trigger value="control" disabled={!mpvState.isRunning}
          >Control</Tabs.Trigger
        >
        <Tabs.Trigger value="history">History</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="home">
        <h2 class="text-2xl font-bold mb-4">MPV Player Control</h2>
        <MPVacious />
      </Tabs.Content>
      <Tabs.Content value="how-to">
        <HowTo />
      </Tabs.Content>
      <Tabs.Content value="control">
        <Control isRunning={mpvState.isRunning} />
      </Tabs.Content>
      <Tabs.Content value="history">
        <History />
      </Tabs.Content>
    </Tabs.Root>
  </div>
  <div class="flex gap-4 mt-2 items-center">
    <Button
      placeholder="Select video file..."
      disabled={mpvState.isLoading}
      onclick={async () => {
        mpvState.mediaFile = await open({
          multiple: false,
          directory: false,
          filters: [
            {
              name: "Video File",
              extensions: ["mp4", "mkv", "avi", "mov", "flv"],
            },
          ],
        });
        mpvState.watchHistoryId = null;
        mpvState.restorePoint = null;
        if (mpvState.isRunning && mpvState.mediaFile) {
          await command("loadfile", [mpvState.mediaFile]);
        }
      }}
      ><IconFile /> Select Video
    </Button>
    {#if !mpvState.isRunning}
      <Button
        onclick={startMPV}
        disabled={mpvState.isRunning || mpvState.isLoading}
      >
        {#if mpvState.isLoading}
          <Spinner />{/if}
        Start mpv
      </Button>
    {:else}
      <Button
        onclick={() => stopMPV()}
        disabled={!mpvState.isRunning || mpvState.isLoading}
        variant="destructive"
      >
        {#if mpvState.isLoading}
          <Spinner />{/if}
        Stop mpv
      </Button>
    {/if}
    <div class="text-sm text-muted-foreground">
      {#if mpvState.mediaFile}
        Selected file: {mpvState.mediaFile.split("/").pop()}
      {:else}
        No file selected
      {/if}
    </div>
  </div>
</div>
