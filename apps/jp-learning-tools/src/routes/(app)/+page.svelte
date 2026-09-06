<script lang="ts">
  import kanjiState from "../../stores/kanjiBank.svelte";
  import { statusState } from "../../stores/statusState.svelte";
  import ProgressTable from "../../components/ProgressTable.svelte";

  let kanji = $derived(kanjiState());
</script>

<div class="w-full p-4 h-full min-h-0 flex flex-col">
  {#if statusState.manifestStatus === "error"}
    <div class="mb-4 p-4 bg-red-100 text-red-800 rounded">
      <p class="font-bold">Error loading manifest</p>
      <p>{statusState.manifestError}</p>
    </div>
  {/if}
  <div class="shrink-0">
    <table class="table-auto border-collapse border`">
      <thead class="">
        <tr class="text-left">
          <th class="px-4 py-2">Source</th>
          <th class="px-4 py-2 text-pink-500">Kanji</th>
          <th class="px-4 py-2 text-purple-500">Vocab</th>
          <th class="px-4 py-2">Total</th>
        </tr>
      </thead>
      <tbody class="divide-y">
        <tr class="">
          <td class="px-4 py-2">Anki</td>
          <td class="px-4 py-2 text-pink-500">{kanji.totalKanjiSource.anki}</td>
          <td class="px-4 py-2 text-purple-500"
            >{kanji.totalVocabSource.anki}</td
          >
          <td class="px-4 py-2">{kanji.totalSource.anki}</td>
        </tr>
        <tr class="">
          <td class="px-4 py-2">WaniKani</td>
          <td class="px-4 py-2 text-pink-500"
            >{kanji.totalKanjiSource.wanikani}</td
          >
          <td class="px-4 py-2 text-purple-500"
            >{kanji.totalVocabSource.wanikani}</td
          >
          <td class="px-4 py-2">{kanji.totalSource.wanikani}</td>
        </tr>
        <tr>
          <td class="px-4 py-2 font-bold">Total</td>
          <td class="px-4 py-2 font-bold text-pink-500">{kanji.kanji.length}</td
          >
          <td class="px-4 py-2 font-bold text-purple-500"
            >{kanji.vocab.length}</td
          >
          <td class="px-4 py-2 font-bold">{kanji.total}</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="mt-4 min-h-0 flex-1">
    <ProgressTable />
  </div>
</div>
