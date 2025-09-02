<script lang="ts">
  import { Skeleton } from "$lib/components/ui/skeleton/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import Loader2Icon from "@lucide/svelte/icons/loader-2";
  import { set, get, save } from "./store";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { kanjiImporter } from "..";
  import { onMount } from "svelte";
  import { alertState } from "../../../stores/alertState.svelte";
  import KanjiBank from "../kanji-bank";
  import { formatDate } from "$lib/date";

  type SaveState = "IDLE" | "SAVING" | "SAVED";
  let saveState: SaveState = $state("IDLE");

  let apiKey: string | null | undefined = $state(undefined);
  let lastSync: Date | null | undefined = $state(undefined);
  let inputValue: string = $state("");

  let isSaveDisabled = $derived(saveState !== "IDLE" || inputValue === apiKey);

  onMount(async () => {
    const [storedApiKey, storedLastSync] = await Promise.all([
      get("apiKey"),
      get("lastSync"),
    ]);
    apiKey = storedApiKey;
    inputValue = storedApiKey ?? "";
    lastSync = storedLastSync ? new Date(storedLastSync) : null;
  });
</script>

<form
  onsubmit={async (event) => {
    event.preventDefault();
    saveState = "SAVING";
    const formData = new FormData(event.target as HTMLFormElement);

    await set("apiKey", formData.get("api-key") as string);
    await save();
    apiKey = formData.get("api-key") as string;
    saveState = "SAVED";
    await setTimeout(() => {
      saveState = "IDLE";
    }, 750);
  }}
>
  {#if apiKey === undefined || lastSync === undefined}
    <!-- <Skeleton size="sm" class="my-8" /> -->
    <Skeleton class="h-[20px] w-[100px] rounded-full my-8" />
  {:else}
    <div class="flex w-full max-w-sm flex-col gap-1.5">
      <Label for="api-key" class="mb-2">API Key</Label>
      <Input
        type="text"
        id="api-key"
        name="api-key"
        placeholder="WaniKani API Key"
        required
        bind:value={inputValue}
      />
      <p class="text-muted-foreground text-sm">
        Your API key can be found <button
          onclick={async () =>
            openUrl("https://www.wanikani.com/settings/personal_access_tokens")}
          class="text-blue-400 font-medium hover:underline">here</button
        >.
      </p>
    </div>
    <div class="flex items-center mt-4 gap-x-4">
      <Button
        class={`${isSaveDisabled ? "" : "bg-green-500 hover:bg-green-600"}`}
        disabled={isSaveDisabled}>Save</Button
      >
      <Button
        disabled={saveState !== "IDLE" || !apiKey}
        onclick={async () => {
          saveState = "SAVING";
          try {
            const result = await (
              await (
                await kanjiImporter["wanikani"]
              )(apiKey)
            ).load();
            const changelog = await KanjiBank.batchKanji(result);
            const lastSyncDate = new Date();
            await set("lastSync", lastSyncDate);
            lastSync = lastSyncDate;
            alertState.alert = {
              alertTitle: "Success",
              alertMessage: `Updated ${Object.values(changelog).length} kanji from WaniKani.`,
              alertType: "success",
            };
          } catch (error) {
            alertState.alert = {
              alertTitle: "Failed to update WaniKani data",
              alertMessage: `${error}`,
              alertType: "error",
            };
            console.error(error);
          }
          saveState = "SAVED";
          await setTimeout(() => {
            saveState = "IDLE";
          }, 750);
        }}
        class="bg-blue-400 hover:bg-blue-600"
      >
        {#if saveState !== "IDLE"}
          <Loader2Icon class="animate-spin" />
        {/if}
        Update
      </Button>
      <p>Last updated: {formatDate(lastSync) ?? "never"}</p>
    </div>
    <div class="flex items-center mt-6"></div>
  {/if}
</form>
