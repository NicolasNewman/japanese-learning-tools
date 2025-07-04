import { LazyStore } from '@tauri-apps/plugin-store';

const STORE_KEY = 'wanikani-settings';
type WaniKaniSettingsStore = {
    'apiKey': string;
    'lastSync': Date | null;
}
const store = new LazyStore(STORE_KEY);

async function delay(msecs: number) {
  return new Promise((resolve) => setTimeout(resolve, msecs));
}
export const get = async <T extends keyof WaniKaniSettingsStore>(key: T, timeout = 0): Promise<WaniKaniSettingsStore[T] | null> => {
    if (timeout > 0) {
        await delay(timeout)
    }
    return await store.get(key) ?? null;
};

export const set = async <T extends keyof WaniKaniSettingsStore>(key: T, value: WaniKaniSettingsStore[T]): Promise<void> => {
    await store.set(key, value);
};

export const save = async (): Promise<void> => {
    await store.save();
}