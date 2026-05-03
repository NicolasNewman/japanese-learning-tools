<script lang="ts">
  import { Skeleton } from "$lib/components/ui/skeleton/index.js";
  import Loader2Icon from "@lucide/svelte/icons/loader-2";
  import { Button } from "$lib/components/ui/button/index.js";
  import { set, get, save } from "./store";
  import { onMount } from "svelte";
  import { kanjiImporter } from "..";
  import { alertState } from "../../../stores/alertState.svelte";
  import KanjiBank from "../kanji-bank";
  import { formatDate } from "$lib/date";
  import { YankiConnect } from "yanki-connect";
  import { fetch } from "@tauri-apps/plugin-http";
  import AnkiConnectClient from "$lib/ankiApi";
  import Checkbox from "$lib/components/ui/checkbox/checkbox.svelte";
  import Select from "$lib/components/app-select.svelte";

  type SaveState = "IDLE" | "SAVING" | "SAVED";
  let saveState: SaveState = $state("IDLE");

  let lastSync: Date | null | undefined = $state(undefined);
  let syncedModels: { [key: string]: string } | null | undefined =
    $state(undefined);
  let syncedDecks: Set<string> | null | undefined = $state(undefined);

  let isSaveDisabled = $derived(saveState !== "IDLE");

  // let client = AnkiConnectClient.getInstance();
  // const decks = AnkiConnectClient.getDecks();
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
          {#each Object.entries(models)
            .filter(([_, { count }]) => count > 0)
            .sort(([_1, { count: a }], [_2, { count: b }]) => b - a) as [modelName, { count, fields }]}
            <div class="flex items-center justify-between">
              <Checkbox
                id={modelName}
                value={modelName}
                name={modelName}
                checked={syncedModels?.hasOwnProperty(modelName) ?? false}
                onCheckedChange={(checked) => {
                  if (checked) {
                    syncedModels = { ...syncedModels, [modelName]: modelName };
                  } else {
                    const { [modelName]: _, ...rest } = syncedModels ?? {};
                    syncedModels = rest;
                  }
                }}
                disabled={saveState !== "IDLE"}
              />
              <p>{modelName} ({count})</p>
              <Select
                options={fields.map((field) => ({
                  value: field,
                  label: field,
                }))}
                value={syncedModels?.[modelName] ?? ""}
                onValueChange={(value) => {
                  if (syncedModels?.hasOwnProperty(modelName)) {
                    syncedModels = { ...syncedModels, [modelName]: value };
                  }
                }}
                name={modelName}
                disabled={saveState !== "IDLE" ||
                  !(syncedModels?.hasOwnProperty(modelName) ?? false)}
              />
            </div>
          {/each}
        </div>
        <div>
          {#each decks as deckName}
            <div class="flex items-center justify-between">
              <Checkbox
                id={deckName}
                value={deckName}
                name={deckName}
                checked={syncedDecks?.has(deckName) ?? false}
                onCheckedChange={(checked) => {
                  if (checked) {
                    syncedDecks?.add(deckName);
                  } else {
                    syncedDecks?.delete(deckName);
                  }
                }}
                disabled={saveState !== "IDLE"}
              />
              <p>{deckName}</p>
            </div>
          {/each}
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
          onclick={async () => {
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
{:catch error}
  <p class="text-red-500">Error loading Anki data: {error}</p>
{/await}
