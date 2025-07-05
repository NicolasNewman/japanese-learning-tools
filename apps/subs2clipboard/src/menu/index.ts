import browser from "webextension-polyfill";
import { sendMessageToTab } from "../lib/background-helper";
import { sessionStore } from "../lib/local-storage";

// ===== copySubsCheckbox =====
const copySubsCheckbox = document.getElementById("copyOnClick") as HTMLInputElement;
sessionStore.get("copyOnClick").then((copyOnClick) => {
    copySubsCheckbox.checked = (copyOnClick as boolean) ?? false;
});
copySubsCheckbox.addEventListener("change", (e) => {
    const enabled = (e.target as HTMLInputElement).checked;
    sessionStore.set("copyOnClick", enabled)
    console.log("Checkbox changed:", enabled);
    browser.tabs.query({ active: true, currentWindow: true }).then((tabs) => {
        sendMessageToTab(tabs[0].id ?? browser.tabs.TAB_ID_NONE, { type: "TOGGLE_SUBS", enabled })
    });
});

// ===== modeSpan =====
const modeSpan = document.getElementById("mode") as HTMLSpanElement;
browser.tabs.query({ active: true, currentWindow: true }).then((tabs) => {
    sendMessageToTab(tabs[0].id ?? browser.tabs.TAB_ID_NONE, { type: "GET_SUPPORTED_SERVICE" }).then((response) => {
        modeSpan.textContent = response || 'None';
    });;
});

// ===== sudachiButton =====
const sudachiButton = document.getElementById("sudachi") as HTMLButtonElement;
sudachiButton.addEventListener("click", () => {
    browser.tabs.query({ active: true, currentWindow: true }).then((tabs) => {
        sendMessageToTab(tabs[0].id ?? browser.tabs.TAB_ID_NONE, { type: "RUN_SUDACHI", tabId: tabs[0].id ?? browser.tabs.TAB_ID_NONE })
    });
});