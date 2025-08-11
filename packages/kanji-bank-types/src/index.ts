export type Level = "Apprentice" | "Guru" | "Master" | "Enlightened" | "Burned";

export type KanjiBankEntry<T> = {
    level: Level;
    type: "kanji" | "vocabulary";
    source: KanjiSource;
    meaning: string;
    metadata: T
}
export type KanjiBankData<T> = Record<
    string,
    KanjiBankEntry<T>
>;

export type KanjiSource = 'wanikani';
export * from './source/wanikani.js'
