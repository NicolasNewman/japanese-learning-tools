import { LazyStore } from "@tauri-apps/plugin-store";

const STORE_KEY = "mpv-settings";

type ScriptOpt = {
  script: string;
  property: string;
  value: string;
};

export const toMpvScriptOpt = (opt: ScriptOpt[]) => {
  return opt
    .map(({ script, property, value }) => `${script}-${property}=${value}`)
    .join(",");
};

const DEFAULT_SCRIPT_OPTS: ScriptOpt[] = [
  {
    script: "subs2srs",
    property: "autoclip_method",
    value: "clipboard",
  },
  {
    script: "subs2srs",
    property: "autoclip",
    value: "yes",
  },
  {
    script: "subs2srs",
    property: "deck_name",
    value: "Active::Japanese::subs2srs",
  },
  {
    script: "subs2srs",
    property: "note_tag",
    value: "subs2srs kanji-first",
  },
];

type MPVSettingsStore = {
  "script-opts": ScriptOpt[];
  "watch-history": Record<
    number,
    { path: string; timestamp: number; subOffset: number }
  >;
};

const DEFAULT_STORE: MPVSettingsStore = {
  "script-opts": DEFAULT_SCRIPT_OPTS,
  "watch-history": {},
};

const store = new LazyStore(STORE_KEY);

async function delay(msecs: number) {
  return new Promise((resolve) => setTimeout(resolve, msecs));
}

export const get = async <T extends keyof MPVSettingsStore>(
  key: T,
  timeout = 0,
): Promise<MPVSettingsStore[T] | null> => {
  if (timeout > 0) {
    await delay(timeout);
  }

  const value = (await store.get(key)) as MPVSettingsStore[T] | undefined;
  console.log(`Getting ${key} from store:`, value);
  if (key === "script-opts" && !value) {
    await store.set(key, DEFAULT_STORE["script-opts"]);
    await store.save();
    return DEFAULT_STORE["script-opts"] as MPVSettingsStore[T];
  } else if (key === "watch-history" && !value) {
    await store.set(key, DEFAULT_STORE["watch-history"]);
    await store.save();
    return DEFAULT_STORE["watch-history"] as MPVSettingsStore[T];
  }

  return (value as MPVSettingsStore[T]) ?? null;
};

export const set = async <T extends keyof MPVSettingsStore>(
  key: T,
  value: MPVSettingsStore[T],
): Promise<void> => {
  await store.set(key, value);
};

export const addToWatchHistory = async (path: string) => {
  const history = (await get("watch-history")) ?? {};
  const id = new Date().getTime();
  await set("watch-history", {
    ...history,
    [id]: { path, timestamp: 0, subOffset: 0 },
  });

  return id;
};

export const updateWatchHistory = async (
  id: number,
  updates: Partial<{ path: string; timestamp: number; subOffset: number }>,
) => {
  const history = (await get("watch-history")) ?? {};
  if (!history[id]) {
    return;
  }

  await set("watch-history", {
    ...history,
    [id]: { ...history[id], ...updates },
  });
};

export const save = async (): Promise<void> => {
  await store.save();
};
