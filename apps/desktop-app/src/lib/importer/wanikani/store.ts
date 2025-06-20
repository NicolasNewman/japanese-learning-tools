import { LazyStore } from '@tauri-apps/plugin-store';

const STORE_KEY = 'wanikani-settings';
type WaniKaniSettingsStore = {
    'apiKey': string;
}
const store = new LazyStore(STORE_KEY);

export const get = async <T extends keyof WaniKaniSettingsStore>(key: T): Promise<WaniKaniSettingsStore[T] | null> => {
    return await store.get(key) ?? null;
};

export const set = async <T extends keyof WaniKaniSettingsStore>(key: T, value: WaniKaniSettingsStore[T]): Promise<void> => {
    await store.set(key, value);
};

export const save = async (): Promise<void> => {
    await store.save();
}