import KanjiBank from "$lib/importer/kanji-bank";
import type { KanjiBankEntry } from "@nicolasnewman/kanji-bank-types";

let _kanjiState: [key: string, value: KanjiBankEntry<unknown>][] = $state([]);

KanjiBank.getAll().then((data) => {
    console.log(data);
  _kanjiState = data || [];
});


type KanjiState = {
    total: number;
    kanji: [key: string, value: KanjiBankEntry<unknown>][];
    vocab: [key: string, value: KanjiBankEntry<unknown>][];
}

let kanjiState: KanjiState = $derived(
    _kanjiState.reduce((prev, curr) => {
        if (curr[1].type === "kanji") {
            prev.kanji.push(curr);
        } else if (curr[1].type === "vocabulary") {
            prev.vocab.push(curr);
        }
        prev.total += 1;
        return prev;
    }, {
        total: 0,
        kanji: [],
        vocab: []
    } as {
        total: number;
        kanji: [key: string, value: KanjiBankEntry<unknown>][];
        vocab: [key: string, value: KanjiBankEntry<unknown>][];
    })
)

export default (() => kanjiState)