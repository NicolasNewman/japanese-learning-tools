import { load, Store } from '@tauri-apps/plugin-store';

export type KanjiBankSource = 'wanikani' | 'anki';
export type KanjiBankData = Record<string, {
    level: number;
    type: 'kanji' | 'vocabulary';
    source: KanjiBankSource;
}>

export default class KanjiBank {
    private static instance: KanjiBank | null = null;
    public static readonly STORE_NAME = 'kanji-bank.json';
    private store: Store | null = null;
    private initialized: boolean = false;
    
    public static async setKanji(kanji: string, level: number, source: KanjiBankSource): Promise<void> {
        (await KanjiBank.getInstance()).set(kanji, { level, source });
    }
    
    public static async getKanji(kanji: string): Promise<KanjiBankData | undefined> {
        return (await KanjiBank.getInstance()).get<KanjiBankData>(kanji);
    }

    private static async getInstance(): Promise<Store> {
        if (!KanjiBank.instance) {
            KanjiBank.instance = new KanjiBank()
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