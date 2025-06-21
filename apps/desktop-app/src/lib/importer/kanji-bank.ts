import { load, Store } from "@tauri-apps/plugin-store";
import type { KanjiSource } from ".";


export type Level = "Apprentice" | "Guru" | "Master" | "Enlightened" | "Burned";

export type KanjiBankEntry = {
  level: Level;
  type: "kanji" | "vocabulary";
  source: KanjiSource;
}
export type KanjiBankData = Record<
  string,
  KanjiBankEntry
>;

export default class KanjiBank {
  private static instance: KanjiBank | null = null;
  public static readonly STORE_NAME = "kanji-bank.json";
  private store: Store | null = null;
  private initialized: boolean = false;

  public static async setKanji(
    kanji: string,
    data: KanjiBankEntry
  ): Promise<void> {
    (await KanjiBank.getInstance()).set(kanji, data);
  }

  public static async batchKanji(kanjiData: KanjiBankData): Promise<KanjiBankData> {
    const storedKanji = (
      await (await KanjiBank.getInstance()).entries<KanjiBankData[""]>()
    ).reduce((prev, [key, value]) => {
      prev[key] = value;
      return prev;
    }, {} as KanjiBankData);
    const changelog: KanjiBankData = {}
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

  public static async getKanji(
    kanji: string
  ): Promise<KanjiBankData | undefined> {
    return (await KanjiBank.getInstance()).get<KanjiBankData>(kanji);
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
