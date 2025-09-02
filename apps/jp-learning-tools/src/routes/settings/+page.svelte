<script lang="ts">
  import { goto } from "$app/navigation";
  import { importerSettingsPage } from "$lib/importer";
  import {
    Tabs,
    TabItem,
    AccordionItem,
    Accordion,
    Checkbox,
  } from "flowbite-svelte";
  import { HomeSolid } from "flowbite-svelte-icons";
  import Settings from "../../stores/settings.svelte";

  const settings = Settings.getAll();
</script>

<Tabs tabStyle="underline">
  <TabItem>
    {#snippet titleSlot()}
      <button onclick={() => goto("/")} class="flex items-center gap-2">
        <HomeSolid size="md" />
      </button>
    {/snippet}
  </TabItem>
  <TabItem open title="General">
    {#await settings}
      <p>Loading settings...</p>
    {:then settings}
      <Checkbox
        checked={settings.debugMode}
        onchange={(e) => Settings.set("debugMode", e.currentTarget.checked)}
        >Enable Debug Mode</Checkbox
      >
    {/await}
  </TabItem>
  <TabItem title="Importers">
    <Accordion flush>
      <AccordionItem>
        {#snippet header()}WaniKani{/snippet}
        <svelte:component this={importerSettingsPage["wanikani"]} />
      </AccordionItem>
      <AccordionItem>
        {#snippet header()}Anki{/snippet}
        <p class="mb-2 text-gray-500 dark:text-gray-400">
          Lorem ipsum dolor sit amet, consectetur adipisicing elit. Illo ab
          necessitatibus sint explicabo ...
        </p>
      </AccordionItem>
    </Accordion>
  </TabItem>
</Tabs>
