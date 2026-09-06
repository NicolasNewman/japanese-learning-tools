<script lang="ts">
  import type {
    AnkiMetadata,
    KanjiSource,
    WaniKaniMetadata,
  } from "@nicolasnewman/kanji-bank-types";
  import kanjiState from "../stores/kanjiBank.svelte";
  import KanjiTable from "./kanji-table/kanji-table.svelte";
  import {
    columns,
    type KanjiTableRow,
  } from "./kanji-table/kanji-table-columns";

  let kanji = $derived.by(() =>
    kanjiState().kanji.reduce(
      (prev, curr) => {
        return {
          ...prev,
          [curr[0]]: true,
        };
      },
      {} as Record<string, boolean>,
    ),
  );

  let data: KanjiTableRow<KanjiSource>[] = $derived.by(() =>
    kanjiState().vocab.reduce<KanjiTableRow<KanjiSource>[]>(
      (prev, [key, value]) => {
        if (/\\text/g.test(key)) return prev;

        const { source, meaning, level, metadata: unknownMetadata } = value;

        if (source === "wanikani") {
          return [
            ...prev,
            {
              value: key,
              kanjiKnown: key.split("").map((k) => kanji[k] || false),
              source,
              meaning,
              level,
              metadata: unknownMetadata as WaniKaniMetadata,
            },
          ];
        }

        return [
          ...prev,
          {
            value: key,
            kanjiKnown: key.split("").map((k) => kanji[k] || false),
            source,
            meaning,
            level,
            metadata: unknownMetadata as AnkiMetadata,
          },
        ];
      },
      [],
    ),
  );
</script>

<div class="h-full min-h-0">
  <KanjiTable {data} {columns} />
</div>
