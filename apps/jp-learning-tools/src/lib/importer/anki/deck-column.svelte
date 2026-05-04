<script lang="ts">
  import Checkbox from "$lib/components/ui/checkbox/checkbox.svelte";

  type SaveState = "IDLE" | "SAVING" | "SAVED";
  let {
    decks,
    syncedDecks = $bindable(),
    saveState,
  }: {
    decks: string[];
    syncedDecks: Set<string> | null | undefined;
    saveState: SaveState;
  } = $props();
</script>

<div class="mb-1 font-semibold text-lg">Decks</div>
<hr class="mb-1" />
{#each decks as deckName}
  <div class="flex items-center gap-x-2 mb-0.5">
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
