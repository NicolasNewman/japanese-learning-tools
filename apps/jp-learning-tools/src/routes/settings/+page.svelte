<script lang="ts">
  import { importerSettingsPage } from "$lib/importer";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import Settings from "../../stores/settings.svelte";
  import { Checkbox } from "$lib/components/ui/checkbox/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import * as Accordion from "$lib/components/ui/accordion/index.js";

  const settings = Settings.getAll();
  let activeTab = $state("general");
</script>

<Tabs.Root bind:value={activeTab} class="mx-2">
  <Tabs.List class="mb-2 -mx-2">
    <Tabs.Trigger value="general">General</Tabs.Trigger>
    <Tabs.Trigger value="import">Importers</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="general">
    {#await settings}
      <p>Loading settings...</p>
    {:then settings}
      <div class="flex items-center gap-3">
        <Checkbox
          id="debugMode"
          checked={settings.debugMode}
          onCheckedChange={(v) => Settings.set("debugMode", v)}
        />
        <Label for="debugMode">Enable debug mode</Label>
      </div>
    {/await}
  </Tabs.Content>
  <Tabs.Content value="import">
    <Accordion.Root type="single" class="w-full">
      <Accordion.Item value="wanikani">
        <Accordion.Trigger>WaniKani</Accordion.Trigger>
        <Accordion.Content class="flex flex-col gap-4 text-balance">
          <svelte:component this={importerSettingsPage["wanikani"]} />
        </Accordion.Content>
      </Accordion.Item>
      <Accordion.Item value="anki">
        <Accordion.Trigger>Anki</Accordion.Trigger>
        <Accordion.Content class="flex flex-col gap-4 text-balance">
          <p class="mb-2 text-gray-500 dark:text-gray-400">
            Lorem ipsum dolor sit amet, consectetur adipisicing elit. Illo ab
            necessitatibus sint explicabo ...
          </p>
        </Accordion.Content>
      </Accordion.Item>
    </Accordion.Root>
  </Tabs.Content>
</Tabs.Root>
