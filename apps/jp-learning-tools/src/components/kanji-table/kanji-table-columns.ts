import type {
  AnkiMetadata,
  KanjiBankData,
  KanjiBankEntry,
  KanjiSource,
  WaniKaniMetadata,
} from "@nicolasnewman/kanji-bank-types";
import { createColumnHelper, renderSnippet } from "@tanstack/svelte-table";
import { type DataTableFeatures } from "./kanji-table-features";
import { createRawSnippet } from "svelte";

export type KanjiTableRow<T extends KanjiSource = KanjiSource> =
  T extends KanjiSource
    ? {
        value: string;
        kanjiKnown: boolean[];
        source: T;
        meaning: string;
        level: string;
        metadata: T extends "wanikani" ? WaniKaniMetadata : AnkiMetadata;
      }
    : never;

const columnHelper = createColumnHelper<
  DataTableFeatures,
  KanjiTableRow<KanjiSource>
>();

export const columns = columnHelper.columns([
  columnHelper.accessor("value", {
    header: "Value",
    cell: ({ row }) => {
      const amountCellSnippet = createRawSnippet<[{ value: string }]>(
        (getValue) => {
          const { value } = getValue();
          // const formatted = formatter.format(value);
          return {
            render: () =>
              `<div class="text-purple-500 text-xl">${value
                .split("")
                .map(
                  (k, i) =>
                    `<span class="${
                      row.original.kanjiKnown[i]
                        ? "underline decoration-pink-500 underline-offset-8 decoration-1"
                        : ""
                    }">${k}</span>`,
                )
                .join("")}</div>`,
          };
        },
      );
      return renderSnippet(amountCellSnippet, { value: row.original.value });
    },
  }),
  columnHelper.accessor("meaning", {
    header: "Meaning",
  }),
  columnHelper.accessor("level", {
    header: "Level",
  }),
  columnHelper.accessor("metadata", {
    header: "Metadata",
    cell: ({ row }) => {
      const metadataCellSnippet = createRawSnippet<
        [{ metadata: WaniKaniMetadata | AnkiMetadata }]
      >((getValue) => {
        // const formatted = JSON.stringify(metadata, null, 2);
        if (row.original.source === "wanikani") {
          const { kanjiData, vocabularyData, level } = row.original.metadata;
          return {
            render: () =>
              `<div class="text-sm text-gray-500"> 
${kanjiData ?? ""}
PoS: ${vocabularyData?.partsOfSpeech.join(", ")}<br>
Reading: ${vocabularyData?.readings.map((r) => r.reading).join(", ")}<br>
Level: ${level}
          </div>`,
            // render: () =>
            //   `<div class="text-sm text-gray-500">
            // ${JSON.stringify(
            //     {
            //       kanjiData,
            //       partOfSpeech: vocabularyData?.partsOfSpeech.join(", "),
            //       vocabReadings: vocabularyData?.readings
            //         .map((r) => r.reading)
            //         .join(", "),
            //       level,
            //     },
            //     null,
            //     2,
            //   )}</div>`,
          };
        } else {
          const {
            deckName,
            stats: { interval, lapses, reps },
            fields,
            modelName,
            cardId,
          } = row.original.metadata;
          return {
            render: () =>
              `<div class="text-sm text-gray-500">
Deck: ${deckName}<br>
Stats: I: ${interval}, L: ${lapses}, R: ${reps}<br>
Model: ${modelName}<br>
Card ID: ${cardId}
</div>`,
          };
          //   render: () =>
          //     `<pre class="text-sm text-gray-500">${JSON.stringify(
          //       {
          //         deckName,
          //         stats: `I: ${interval}, L: ${lapses}, R: ${reps}`,
          //         // fields,
          //         modelName,
          //         cardId,
          //       },
          //       null,
          //       2,
          //     )}</pre>`,
          // };
        }
      });
      return renderSnippet(metadataCellSnippet, {
        metadata: row.original.metadata,
      });
    },
  }),
  columnHelper.accessor("source", {
    header: "Source",
    cell: ({ row }) => {
      const sourceCellSnippet = createRawSnippet<[{ source: KanjiSource }]>(
        (getValue) => {
          const { source } = getValue();
          const formatted = source.charAt(0).toUpperCase() + source.slice(1);
          const url =
            row.original.source === "wanikani"
              ? `<a class="underline text-blue-500" href="${row.original.metadata.url}" target="_blank">${formatted}</a>`
              : formatted;
          return {
            render: () => url,
          };
        },
      );
      return renderSnippet(sourceCellSnippet, { source: row.original.source });
    },
  }),
]);
