<script lang="ts">
  import { type AnkiSettingsStore } from "./store";
  import Checkbox from "$lib/components/ui/checkbox/checkbox.svelte";
  import Select from "$lib/components/app-select.svelte";

  type SaveState = "IDLE" | "SAVING" | "SAVED";
  let {
    models,
    syncedModels = $bindable(),
    saveState,
  }: {
    models: Record<
      string,
      {
        count: number;
        fields: string[];
      }
    >;
    syncedModels: AnkiSettingsStore["syncedModels"] | undefined;
    saveState: SaveState;
  } = $props();
</script>

<div class="mb-1 font-semibold text-lg">Model</div>
<hr class="mb-1" />
<div class="flex items-center justify-between gap-x-2 mb-2">
  <p class="grow font-semibold">Model Name (Card Count)</p>
  <p class="w-[120px] text-center font-semibold">Kanji Field</p>
  <p class="w-[120px] text-center font-semibold">Meaning Field</p>
</div>
{#each Object.entries(models)
  .filter(([_, { count }]) => count > 0)
  .sort(([_1, { count: a }], [_2, { count: b }]) => b - a) as [modelName, { count, fields }]}
  <div class="flex items-center justify-between gap-x-2 mb-1">
    <Checkbox
      id={modelName}
      value={modelName}
      name={modelName}
      checked={syncedModels?.hasOwnProperty(modelName) ?? false}
      onCheckedChange={(checked) => {
        if (checked) {
          syncedModels = {
            ...syncedModels,
            [modelName]: {
              kanji: "",
              meaning: "",
            },
          };
        } else {
          const { [modelName]: _, ...rest } = syncedModels ?? {};
          syncedModels = rest;
        }
      }}
      disabled={saveState !== "IDLE"}
    />
    <p class="grow">{modelName} ({count})</p>
    <Select
      options={fields.map((field) => ({
        value: field,
        label: field,
      }))}
      style="w-[120px]"
      value={syncedModels?.[modelName]?.kanji ?? ""}
      onValueChange={(value) => {
        if (syncedModels?.hasOwnProperty(modelName)) {
          syncedModels = {
            ...syncedModels,
            [modelName]: {
              ...syncedModels[modelName],
              kanji: value,
            },
          };
        }
      }}
      name={`${modelName}-kanji`}
      disabled={saveState !== "IDLE" ||
        !(syncedModels?.hasOwnProperty(modelName) ?? false)}
    />
    <Select
      options={fields.map((field) => ({
        value: field,
        label: field,
      }))}
      style="w-[120px]"
      value={syncedModels?.[modelName]?.meaning ?? ""}
      onValueChange={(value) => {
        if (syncedModels?.hasOwnProperty(modelName)) {
          syncedModels = {
            ...syncedModels,
            [modelName]: {
              ...syncedModels[modelName],
              meaning: value,
            },
          };
        }
      }}
      name={`${modelName}-meaning`}
      disabled={saveState !== "IDLE" ||
        !(syncedModels?.hasOwnProperty(modelName) ?? false)}
    />
  </div>
{/each}
