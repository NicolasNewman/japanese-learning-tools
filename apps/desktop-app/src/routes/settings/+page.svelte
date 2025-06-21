<script lang="ts">
    import { goto } from "$app/navigation";
    import { importerSettingsPage } from "$lib/importer";
    import { Tabs, TabItem, AccordionItem, Accordion } from "flowbite-svelte";
    import { HomeSolid } from "flowbite-svelte-icons";
</script>

<Tabs tabStyle="underline">
    <TabItem>
        {#snippet titleSlot()}
            <button on:click={() => goto('/')} class="flex items-center gap-2">
                <HomeSolid size="md"/>
            </button>
        {/snippet}
    </TabItem>
    <TabItem open title="General">
        <p>General settings content goes here.</p>
    </TabItem>
    <TabItem title="Importers">
        <Accordion>
            <AccordionItem>
                {#snippet header()}WaniKani{/snippet}
                <svelte:component this={importerSettingsPage['wanikani']} />
                <!-- <p class="mb-2 text-gray-500 dark:text-gray-400">
                    Lorem ipsum dolor sit amet, consectetur adipisicing elit.
                    Illo ab necessitatibus sint explicabo ...
                </p> -->
            </AccordionItem>
            <AccordionItem>
                {#snippet header()}Anki{/snippet}
                <p class="mb-2 text-gray-500 dark:text-gray-400">
                    Lorem ipsum dolor sit amet, consectetur adipisicing elit.
                    Illo ab necessitatibus sint explicabo ...
                </p>
            </AccordionItem>
        </Accordion>
    </TabItem>
</Tabs>
