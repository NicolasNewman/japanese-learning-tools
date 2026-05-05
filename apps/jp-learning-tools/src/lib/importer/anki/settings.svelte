<script lang="ts">
  import { Skeleton } from "$lib/components/ui/skeleton/index.js";
  import Loader2Icon from "@lucide/svelte/icons/loader-2";
  import { Button } from "$lib/components/ui/button/index.js";
  import { set, get, type AnkiSettingsStore } from "./store";
  import { onMount } from "svelte";
  import { kanjiImporter } from "..";
  import { alertState } from "../../../stores/alertState.svelte";
  import KanjiBank from "../kanji-bank";
  import { formatDate } from "$lib/date";
  import AnkiConnectClient from "$lib/ankiApi";
  import ModelColumn from "./model-column.svelte";
  import DeckColumn from "./deck-column.svelte";
  import type { SaveState } from "..";

  let saveState: SaveState = $state("IDLE");

  let lastSync: Date | null | undefined = $state(undefined);
  let syncedModels: AnkiSettingsStore["syncedModels"] | undefined =
    $state(undefined);
  let syncedDecks: Set<string> | null | undefined = $state(undefined);

  let isSaveDisabled = $derived(saveState !== "IDLE");

  const ankiData = AnkiConnectClient.getInitialInfo();

  onMount(async () => {
    const [storedLastSync, storedSyncedModels, storedSyncedDecks] =
      await Promise.all([
        get("lastSync"),
        get("syncedModels"),
        get("syncedDecks"),
      ]);
    lastSync = storedLastSync ? new Date(storedLastSync) : null;
    syncedModels = storedSyncedModels ? { ...storedSyncedModels } : {};
    syncedDecks = storedSyncedDecks ? new Set(storedSyncedDecks) : new Set();
  });

  const onUpdate = async () => {
    saveState = "SAVING";

    try {
      if (
        syncedModels &&
        Object.keys(syncedModels).length > 0 &&
        syncedDecks &&
        syncedDecks.size > 0
      ) {
        const result = await (
          await (
            await kanjiImporter["anki"]
          )(syncedModels, Array.from(syncedDecks))
        ).load();
        const changelog = await KanjiBank.batchKanji(result);
        const lastSyncDate = new Date();
        await set("lastSync", lastSyncDate);
        lastSync = lastSyncDate;
        alertState.alert = {
          alertTitle: "Success",
          alertMessage: `Updated ${Object.values(changelog).length} kanji from Anki.`,
          alertType: "success",
        };
      }
    } catch (error) {
      alertState.alert = {
        alertTitle: "Failed to update Anki data",
        alertMessage: `${error}`,
        alertType: "error",
      };
      console.error(error);
    }
    saveState = "SAVED";
    await setTimeout(() => {
      saveState = "IDLE";
    }, 750);
  };
</script>

{#await ankiData}
  Loading...
  <Skeleton class="h-[20px] w-[100px] rounded-full my-8" />
{:then { decks, models }}
  <form
    onsubmit={async (event) => {
      event.preventDefault();
      saveState = "SAVING";
      await set("syncedModels", { ...syncedModels });
      await set("syncedDecks", Array.from(syncedDecks ?? []));
      saveState = "SAVED";
      await setTimeout(() => {
        saveState = "IDLE";
      }, 750);
    }}
  >
    {#if lastSync === undefined || syncedModels === undefined || syncedDecks === undefined}
      <!-- <Skeleton size="sm" class="my-8" /> -->
      Loading...
      <Skeleton class="h-[20px] w-[100px] rounded-full my-8" />
    {:else}
      <div class="flex w-full max-w-sm flex-col gap-1.5"></div>
      <div class="grid grid-cols-2 gap-x-8">
        <div>
          <ModelColumn {models} bind:syncedModels {saveState} />
        </div>
        <div>
          <DeckColumn {decks} bind:syncedDecks {saveState} />
        </div>
      </div>
      <div class="flex items-center mt-4 gap-x-4">
        <Button
          type="submit"
          class={`${isSaveDisabled ? "" : "bg-green-500 hover:bg-green-600"}`}
          disabled={isSaveDisabled}>Save</Button
        >
        <Button
          disabled={saveState !== "IDLE"}
          onclick={onUpdate}
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
{:catch error}
  <p class="text-red-500">Error loading Anki data: {error}</p>
{/await}
