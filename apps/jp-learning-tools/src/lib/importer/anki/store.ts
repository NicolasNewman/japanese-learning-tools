import { LazyStore } from "@tauri-apps/plugin-store";

const STORE_KEY = "anki-settings";
type AnkiSettingsStore = {
  lastSync: Date | null;
  syncedModels: { [key: string]: string } | null;
  syncedDecks: string[] | null;
};
const store = new LazyStore(STORE_KEY);

async function delay(msecs: number) {
  return new Promise((resolve) => setTimeout(resolve, msecs));
}

export const get = async <T extends keyof AnkiSettingsStore>(
  key: T,
  timeout = 0,
): Promise<AnkiSettingsStore[T] | null> => {
  if (timeout > 0) {
    await delay(timeout);
  }
  return (await store.get(key)) ?? null;
};

export const set = async <T extends keyof AnkiSettingsStore>(
  key: T,
  value: AnkiSettingsStore[T],
): Promise<void> => {
  await store.set(key, value);
};

export const save = async (): Promise<void> => {
  await store.save();
};
