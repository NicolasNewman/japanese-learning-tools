import KanjiBank from "$lib/importer/kanji-bank";
import type {
  KanjiBankEntry,
  KanjiSource,
} from "@nicolasnewman/kanji-bank-types";

let _kanjiState: [key: string, value: KanjiBankEntry<unknown>][] = $state([]);

KanjiBank.getAll().then((data) => {
  console.log(data);
  _kanjiState = data || [];
});

type KanjiState = {
  total: number;
  totalSource: Record<KanjiSource, number>;
  totalVocabSource: Record<KanjiSource, number>;
  totalKanjiSource: Record<KanjiSource, number>;
  kanji: [key: string, value: KanjiBankEntry<unknown>][];
  kanjiLookup: Record<string, boolean>;
  vocab: [key: string, value: KanjiBankEntry<unknown>][];
};

let kanjiState: KanjiState = $derived(
  _kanjiState.reduce(
    (prev, curr) => {
      if (curr[1].type === "kanji") {
        prev.kanji.push(curr);
        prev.kanjiLookup[curr[0]] = true;
        prev.totalSource[curr[1].source] += 1;
        prev.totalKanjiSource[curr[1].source] += 1;
      } else if (curr[1].type === "vocabulary") {
        prev.vocab.push(curr);
        prev.totalSource[curr[1].source] += 1;
        prev.totalVocabSource[curr[1].source] += 1;
      }
      prev.total += 1;
      return prev;
    },
    {
      total: 0,
      totalSource: {
        anki: 0,
        wanikani: 0,
      },
      totalVocabSource: {
        anki: 0,
        wanikani: 0,
      },
      totalKanjiSource: {
        anki: 0,
        wanikani: 0,
      },
      kanji: [],
      kanjiLookup: {},
      vocab: [],
    } as {
      total: number;
      totalSource: Record<KanjiSource, number>;
      totalVocabSource: Record<KanjiSource, number>;
      totalKanjiSource: Record<KanjiSource, number>;
      kanji: [key: string, value: KanjiBankEntry<unknown>][];
      kanjiLookup: Record<string, boolean>;
      vocab: [key: string, value: KanjiBankEntry<unknown>][];
    },
  ),
);

export default () => kanjiState;
