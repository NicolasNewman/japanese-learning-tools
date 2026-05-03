export type Level = "Apprentice" | "Guru" | "Master" | "Enlightened" | "Burned";

export type KanjiBankEntry<T> = {
  level: Level;
  type: "kanji" | "vocabulary";
  source: KanjiSource;
  meaning: string;
  metadata: T;
};
export type KanjiBankData<T> = Record<string, KanjiBankEntry<T>>;

export const containsKanji = (str: string) => {
  return /[\p{Script=Han}]/u.test(str);
};

export type KanjiSource = "wanikani" | "anki";
export * from "./source/wanikani.js";
export * from "./source/anki.js";
