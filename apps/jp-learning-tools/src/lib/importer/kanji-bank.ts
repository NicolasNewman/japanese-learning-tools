import { load, Store } from "@tauri-apps/plugin-store";
import type { KanjiSource } from ".";


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

export default class KanjiBank {
  private static instance: KanjiBank | null = null;
  public static readonly STORE_NAME = "kanji-bank.json";
  private store: Store | null = null;
  private initialized: boolean = false;

  public static async setKanji<T>(
    kanji: string,
    data: KanjiBankEntry<T>
  ): Promise<void> {
    (await KanjiBank.getInstance()).set(kanji, data);
  }

  public static async batchKanji<T>(kanjiData: KanjiBankData<T>): Promise<KanjiBankData<T>> {
    const storedKanji = (
      await (await KanjiBank.getInstance()).entries<KanjiBankData<T>[""]>()
    ).reduce((prev, [key, value]) => {
      prev[key] = value;
      return prev;
    }, {} as KanjiBankData<T>);
    const changelog: KanjiBankData<T> = {}
    const updatePromises: Promise<void>[] = [];
    Object.entries(kanjiData).forEach(([kanji, data]) => {
      const storedVersion = storedKanji[kanji];
      if (!storedVersion || (storedVersion.source === data.source && storedVersion.level < data.level)) {
        changelog[kanji] = data;
        updatePromises.push(KanjiBank.setKanji(kanji, data));
        storedKanji[kanji] = data
      }
    });
    await Promise.all(updatePromises);
    await (await KanjiBank.getInstance()).save();
    return changelog;
  }

  public static async getKanji<T>(
    kanji: string
  ): Promise<KanjiBankData<T> | undefined> {
    return (await KanjiBank.getInstance()).get<KanjiBankData<T>>(kanji);
  }

  private static async getInstance(): Promise<Store> {
    if (!KanjiBank.instance) {
      KanjiBank.instance = new KanjiBank();
    }
    return KanjiBank.instance.getStore();
  }

  private async initialize(): Promise<void> {
    if (!this.initialized) {
      this.store = await load(KanjiBank.STORE_NAME);
      this.initialized = true;
    }
  }

  private async getStore(): Promise<Store> {
    if (!this.initialized) {
      await this.initialize();
    }
    return this.store!;
  }
}
