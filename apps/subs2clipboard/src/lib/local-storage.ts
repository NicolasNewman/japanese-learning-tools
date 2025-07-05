import browser from "webextension-polyfill";

type LocalStorage = {
    copyOnClick: boolean;
};

const defaultStorage: LocalStorage = {
    copyOnClick: false
};

const { get, set } = browser.storage.local;

export const sessionStore = {
    get: async <T extends keyof LocalStorage>(key: T): Promise<LocalStorage[T]> => {
        const item = await get({ [key]: defaultStorage[key] });
        return item[key] as unknown as LocalStorage[T];
    },
    set: async <T extends keyof LocalStorage>(key: T, value: LocalStorage[T]) => {
        await set({ [key]: value });
    }
};