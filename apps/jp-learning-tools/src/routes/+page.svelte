<script lang="ts">
  import { goto } from "$app/navigation";
  import KanjiBank from "$lib/importer/kanji-bank";
  import kanjiState from "../stores/kanjiBank.svelte";

  let kanji = $derived(kanjiState());
</script>

<main class="container p-4">
  <div class="mb-4">
    <p>Kanji known: {kanji.kanji.length}</p>
    <p>Vocab known: {kanji.vocab.length}</p>
    <p>Total: {kanji.total}</p>
  </div>
</main>
