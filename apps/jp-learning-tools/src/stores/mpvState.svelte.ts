import type { UnlistenFn } from "@tauri-apps/api/event";

type MpvState = {
  isRunning: boolean;
  isLoading: boolean;
  /** Current playback timestamp in seconds */
  timestamp: number;
  /** Current subtitle offset in seconds */
  subOffset: number;
  mediaFile: string | null;
  unlisten: null | UnlistenFn;
  unlistenEvents: null | UnlistenFn;
  watchHistoryId: number | null;
  /** Restore point for resuming playback when a media file is loaded from history */
  restorePoint: {
    timestamp: number;
    subOffset: number;
  } | null;
};

export const mpvState: MpvState = $state({
  isRunning: false,
  isLoading: false,
  timestamp: 0,
  subOffset: 0,
  mediaFile: null,
  unlisten: null,
  unlistenEvents: null,
  watchHistoryId: null,
  restorePoint: null,
});
