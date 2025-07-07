import { onRuntimeMessage } from "./lib/content-helper";
import { sessionStore } from "./lib/local-storage";

// let initialized = false;
let debugModeEnabled = false;
const initialize = () => {
    // @ts-expect-error custom property on window
    if (window.__debugModeInitialized) return;
    // @ts-expect-error custom property on window
    window.__debugModeInitialized = true;
    sessionStore.get("debugModeEnabled").then((debugMode) => {
        debugModeEnabled = debugMode;
        log("Debug mode enabled:", debugMode);
    });
    onRuntimeMessage((msg) => {
        if (msg.type === "DEBUG_MODE_CHANGED") {
            debugModeEnabled = msg.enabled;
            log(`Debug mode: ${debugModeEnabled}`);
        }
    });
}

export const log = (...args: any[]) => {
    if (debugModeEnabled) {
        console.log("[subs2clipboard]", ...args);
    }
};

initialize();