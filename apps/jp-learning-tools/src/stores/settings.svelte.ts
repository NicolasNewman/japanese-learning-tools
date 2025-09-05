import type { UnlistenFn } from "@tauri-apps/api/event";
import { load, Store } from "@tauri-apps/plugin-store";

export interface SettingsData {
  debugMode: boolean;
}

const DEFAULT_SETTINGS: SettingsData = {
  debugMode: false,
};

export default class Settings {
  private static instance: Settings | null = null;
  public static readonly STORE_NAME = "settings.json";
  private store: Store | null = null;
  private initialized: boolean = false;

  public static async getAll(): Promise<SettingsData> {
    const entries = (await (await Settings.getInstance()).entries()) as [
      key: keyof SettingsData,
      value: SettingsData[keyof SettingsData]
    ][];

    return entries.reduce((prev, [key, value]) => {
      prev[key] = value ?? DEFAULT_SETTINGS[key];
      return prev;
    }, {} as SettingsData);
  }

  public static async get<T extends keyof SettingsData>(
    key: T
  ): Promise<SettingsData[T]> {
    return (
      (await (await Settings.getInstance()).get<SettingsData[T]>(key)) ??
      DEFAULT_SETTINGS[key]
    );
  }

  public static async onChange<T extends keyof SettingsData>(
    key: T,
    callback: (newValue: SettingsData[T]) => void
  ): Promise<UnlistenFn> {
    const store = await Settings.getInstance();
    return store.onChange((_key, newValue) => {
      if (_key === key) {
        callback(newValue as SettingsData[T]);
      }
    });
  }

  public static async set<T extends keyof SettingsData>(
    key: T,
    value: SettingsData[T]
  ): Promise<void> {
    await (await Settings.getInstance()).set(key, value);
  }

  private static async getInstance(): Promise<Store> {
    if (!Settings.instance) {
      Settings.instance = new Settings();
    }
    return Settings.instance.getStore();
  }

  private async initialize(): Promise<void> {
    if (!this.initialized) {
      this.store = await load(Settings.STORE_NAME);
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
