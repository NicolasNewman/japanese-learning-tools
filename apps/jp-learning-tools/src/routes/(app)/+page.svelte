<script lang="ts">
  import kanjiState from "../../stores/kanjiBank.svelte";
  import { statusState } from "../../stores/statusState.svelte";

  let kanji = $derived(kanjiState());
</script>

<main class="container p-4">
  <div class="mb-4">
    <p>Kanji known: {kanji.kanji.length}</p>
    <p>Vocab known: {kanji.vocab.length}</p>
    <p>
      Total: {kanji.total} (Anki: {kanji.totalSource.anki}, WaniKani: {kanji
        .totalSource.wanikani})
    </p>
  </div>
  {#if statusState.manifestStatus === "error"}
    <div class="mb-4 p-4 bg-red-100 text-red-800 rounded">
      <p class="font-bold">Error loading manifest</p>
      <p>{statusState.manifestError}</p>
    </div>
  {/if}
</main>
