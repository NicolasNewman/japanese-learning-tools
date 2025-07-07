import { onRuntimeMessage } from "./lib/content-helper";
import { sessionStore } from "./lib/local-storage";

declare global {
    interface Window {
        __debugModeInitialized?: boolean;
        __debugModeEnabled?: boolean;
    }
}

const initialize = () => {
    if (window.__debugModeInitialized) return;
    window.__debugModeInitialized = true;
    sessionStore.get("debugModeEnabled").then((debugMode) => {
        window.__debugModeEnabled = debugMode;
        // debugModeEnabled = debugMode;
        log("Debug mode enabled:", window.__debugModeEnabled);
    });
    onRuntimeMessage((msg) => {
        if (msg.type === "DEBUG_MODE_CHANGED") {
            window.__debugModeEnabled = msg.enabled;
            log(`Debug mode: ${window.__debugModeEnabled}`);
        }
    });
}

export const log = (...args: any[]) => {
    if (window.__debugModeEnabled) {
        console.log("[subs2clipboard]", ...args);
    }
};

initialize();