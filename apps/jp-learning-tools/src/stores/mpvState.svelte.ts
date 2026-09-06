import type { UnlistenFn } from "@tauri-apps/api/event";

type MpvState = {
  isRunning: boolean;
  isLoading: boolean;
  mediaFile: string | null;
  unlisten: null | UnlistenFn;
  unlistenEvents: null | UnlistenFn;
  watchHistoryId: number | null;
  restorePoint: {
    timestamp: number;
    subOffset: number;
  } | null;
};

export const mpvState: MpvState = $state({
  isRunning: false,
  isLoading: false,
  mediaFile: null,
  unlisten: null,
  unlistenEvents: null,
  watchHistoryId: null,
  restorePoint: null,
});
