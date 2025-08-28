import browser from "webextension-polyfill";
import './styles.scss';
import { Tab } from 'bootstrap';
import { sendMessageToTab } from "../lib/background-helper";
import { sessionStore } from "../lib/local-storage";

// ===== nav-tabs =====
const tabTriggerList = Array.from(document.querySelectorAll('#nav-tab button[data-bs-toggle="tab"]'));
tabTriggerList.forEach(tabTriggerEl => {
    new Tab(tabTriggerEl);
});

// ===== copySubsCheckbox =====
const copySubsCheckbox = document.getElementById("checkboxCopySubs") as HTMLInputElement;
sessionStore.get("copySubsEnabled").then((copySubsEnabled) => {
    copySubsCheckbox.checked = (copySubsEnabled as boolean) ?? false;
});
copySubsCheckbox.addEventListener("change", (e) => {
    const enabled = (e.target as HTMLInputElement).checked;
    sessionStore.set("copySubsEnabled", enabled)
    console.log("Checkbox changed:", enabled);
    browser.tabs.query({ active: true, currentWindow: true }).then((tabs) => {
        sendMessageToTab(tabs[0].id ?? browser.tabs.TAB_ID_NONE, { type: "TOGGLE_SUBS", enabled })
    });
});

// ===== debugModeCheckbox =====
const debugModeCheckbox = document.getElementById("checkboxDebugMode") as HTMLInputElement;
sessionStore.get("debugModeEnabled").then((debugMode) => {
    debugModeCheckbox.checked = (debugMode as boolean) ?? false;
});
debugModeCheckbox.addEventListener("change", (e) => {
    const enabled = (e.target as HTMLInputElement).checked;
    sessionStore.set("debugModeEnabled", enabled)
    console.log("Checkbox changed:", enabled);
    browser.tabs.query({ active: true, currentWindow: true }).then((tabs) => {
        sendMessageToTab(tabs[0].id ?? browser.tabs.TAB_ID_NONE, { type: "DEBUG_MODE_CHANGED", enabled })
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